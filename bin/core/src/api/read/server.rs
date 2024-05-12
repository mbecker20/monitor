use std::{
  collections::{HashMap, HashSet},
  sync::{Arc, OnceLock},
};

use anyhow::{anyhow, Context};
use async_timing_util::{
  get_timelength_in_ms, unix_timestamp_ms, FIFTEEN_SECONDS_MS,
};
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    deployment::ContainerSummary,
    permission::PermissionLevel,
    server::{
      docker_image::ImageSummary, docker_network::DockerNetwork,
      Server, ServerActionState, ServerListItem, ServerState,
    },
    user::User,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use periphery_client::api::{self, GetAccountsResponse};
use resolver_api::{Resolve, ResolveToString};
use tokio::sync::Mutex;

use crate::{
  config::core_config,
  helpers::periphery_client,
  resource,
  state::{action_states, db_client, server_status_cache, State},
};

#[async_trait]
impl Resolve<GetServersSummary, User> for State {
  async fn resolve(
    &self,
    GetServersSummary {}: GetServersSummary,
    user: User,
  ) -> anyhow::Result<GetServersSummaryResponse> {
    let servers =
      resource::list_for_user::<Server>(Default::default(), &user)
        .await?;
    let mut res = GetServersSummaryResponse::default();
    for server in servers {
      res.total += 1;
      match server.info.state {
        ServerState::Ok => {
          res.healthy += 1;
        }
        ServerState::NotOk => {
          res.unhealthy += 1;
        }
        ServerState::Disabled => {
          res.disabled += 1;
        }
      }
    }
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetPeripheryVersion, User> for State {
  async fn resolve(
    &self,
    req: GetPeripheryVersion,
    user: User,
  ) -> anyhow::Result<GetPeripheryVersionResponse> {
    let server = resource::get_check_permissions::<Server>(
      &req.server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let version = server_status_cache()
      .get(&server.id)
      .await
      .map(|s| s.version.clone())
      .unwrap_or(String::from("unknown"));
    Ok(GetPeripheryVersionResponse { version })
  }
}

#[async_trait]
impl Resolve<GetServer, User> for State {
  async fn resolve(
    &self,
    req: GetServer,
    user: User,
  ) -> anyhow::Result<Server> {
    resource::get_check_permissions::<Server>(
      &req.server,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListServers, User> for State {
  async fn resolve(
    &self,
    ListServers { query }: ListServers,
    user: User,
  ) -> anyhow::Result<Vec<ServerListItem>> {
    resource::list_for_user::<Server>(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetServerState, User> for State {
  async fn resolve(
    &self,
    GetServerState { server }: GetServerState,
    user: User,
  ) -> anyhow::Result<GetServerStateResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status = server_status_cache()
      .get(&server.id)
      .await
      .ok_or(anyhow!("did not find cached status for server"))?;
    let response = GetServerStateResponse {
      status: status.state,
    };
    Ok(response)
  }
}

#[async_trait]
impl Resolve<GetServerActionState, User> for State {
  async fn resolve(
    &self,
    GetServerActionState { server }: GetServerActionState,
    user: User,
  ) -> anyhow::Result<ServerActionState> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .server
      .get(&server.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

// This protects the peripheries from spam requests
const SYSTEM_INFO_EXPIRY: u128 = FIFTEEN_SECONDS_MS;
type SystemInfoCache = Mutex<HashMap<String, Arc<(String, u128)>>>;
fn system_info_cache() -> &'static SystemInfoCache {
  static SYSTEM_INFO_CACHE: OnceLock<SystemInfoCache> =
    OnceLock::new();
  SYSTEM_INFO_CACHE.get_or_init(Default::default)
}

#[async_trait]
impl ResolveToString<GetSystemInformation, User> for State {
  async fn resolve_to_string(
    &self,
    GetSystemInformation { server }: GetSystemInformation,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    let mut lock = system_info_cache().lock().await;
    let res = match lock.get(&server.id) {
      Some(cached) if cached.1 > unix_timestamp_ms() => {
        cached.0.clone()
      }
      _ => {
        let stats = periphery_client(&server)?
          .request(api::stats::GetSystemInformation {})
          .await?;
        let res = serde_json::to_string(&stats)?;
        lock.insert(
          server.id,
          (res.clone(), unix_timestamp_ms() + SYSTEM_INFO_EXPIRY)
            .into(),
        );
        res
      }
    };
    Ok(res)
  }
}

#[async_trait]
impl ResolveToString<GetSystemStats, User> for State {
  async fn resolve_to_string(
    &self,
    GetSystemStats { server }: GetSystemStats,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status =
      server_status_cache().get(&server.id).await.with_context(
        || format!("did not find status for server at {}", server.id),
      )?;
    let stats = status
      .stats
      .as_ref()
      .context("server stats not available")?;
    let stats = serde_json::to_string(&stats)?;
    Ok(stats)
  }
}

// This protects the peripheries from spam requests
const PROCESSES_EXPIRY: u128 = FIFTEEN_SECONDS_MS;
type ProcessesCache = Mutex<HashMap<String, Arc<(String, u128)>>>;
fn processes_cache() -> &'static ProcessesCache {
  static PROCESSES_CACHE: OnceLock<ProcessesCache> = OnceLock::new();
  PROCESSES_CACHE.get_or_init(Default::default)
}

#[async_trait]
impl ResolveToString<GetSystemProcesses, User> for State {
  async fn resolve_to_string(
    &self,
    GetSystemProcesses { server }: GetSystemProcesses,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let mut lock = processes_cache().lock().await;
    let res = match lock.get(&server.id) {
      Some(cached) if cached.1 > unix_timestamp_ms() => {
        cached.0.clone()
      }
      _ => {
        let stats = periphery_client(&server)?
          .request(api::stats::GetSystemProcesses {})
          .await?;
        let res = serde_json::to_string(&stats)?;
        lock.insert(
          server.id,
          (res.clone(), unix_timestamp_ms() + PROCESSES_EXPIRY)
            .into(),
        );
        res
      }
    };
    Ok(res)
  }
}

const STATS_PER_PAGE: i64 = 500;

#[async_trait]
impl Resolve<GetHistoricalServerStats, User> for State {
  async fn resolve(
    &self,
    GetHistoricalServerStats {
      server,
      granularity,
      page,
    }: GetHistoricalServerStats,
    user: User,
  ) -> anyhow::Result<GetHistoricalServerStatsResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let granularity =
      get_timelength_in_ms(granularity.to_string().parse().unwrap())
        as i64;
    let mut ts_vec = Vec::<i64>::new();
    let curr_ts = unix_timestamp_ms() as i64;
    let mut curr_ts = curr_ts
      - curr_ts % granularity
      - granularity * STATS_PER_PAGE * page as i64;
    for _ in 0..STATS_PER_PAGE {
      ts_vec.push(curr_ts);
      curr_ts -= granularity;
    }

    let stats = find_collect(
      &db_client().await.stats,
      doc! {
        "sid": server.id,
        "ts": { "$in": ts_vec },
      },
      FindOptions::builder()
        .sort(doc! { "ts": -1 })
        .skip(page as u64 * STATS_PER_PAGE as u64)
        .limit(STATS_PER_PAGE)
        .build(),
    )
    .await
    .context("failed to pull stats from db")?;
    let next_page = if stats.len() == STATS_PER_PAGE as usize {
      Some(page + 1)
    } else {
      None
    };
    let res = GetHistoricalServerStatsResponse { stats, next_page };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetDockerImages, User> for State {
  async fn resolve(
    &self,
    GetDockerImages { server }: GetDockerImages,
    user: User,
  ) -> anyhow::Result<Vec<ImageSummary>> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    periphery_client(&server)?
      .request(api::build::GetImageList {})
      .await
  }
}

#[async_trait]
impl Resolve<GetDockerNetworks, User> for State {
  async fn resolve(
    &self,
    GetDockerNetworks { server }: GetDockerNetworks,
    user: User,
  ) -> anyhow::Result<Vec<DockerNetwork>> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    periphery_client(&server)?
      .request(api::network::GetNetworkList {})
      .await
  }
}

#[async_trait]
impl Resolve<GetDockerContainers, User> for State {
  async fn resolve(
    &self,
    GetDockerContainers { server }: GetDockerContainers,
    user: User,
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    periphery_client(&server)?
      .request(api::container::GetContainerList {})
      .await
  }
}

#[async_trait]
impl Resolve<GetAvailableAccounts, User> for State {
  async fn resolve(
    &self,
    GetAvailableAccounts { server }: GetAvailableAccounts,
    user: User,
  ) -> anyhow::Result<GetAvailableAccountsResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    let GetAccountsResponse { github, docker } =
      periphery_client(&server)?
        .request(api::GetAccounts {})
        .await
        .context("failed to get accounts from periphery")?;

    let mut github_set = HashSet::<String>::new();

    github_set.extend(core_config().github_accounts.keys().cloned());
    github_set.extend(github);

    let mut github = github_set.into_iter().collect::<Vec<_>>();
    github.sort();

    let mut docker_set = HashSet::<String>::new();

    docker_set.extend(core_config().docker_accounts.keys().cloned());
    docker_set.extend(docker);

    let mut docker = docker_set.into_iter().collect::<Vec<_>>();
    docker.sort();

    let res = GetAvailableAccountsResponse { github, docker };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetAvailableSecrets, User> for State {
  async fn resolve(
    &self,
    GetAvailableSecrets { server }: GetAvailableSecrets,
    user: User,
  ) -> anyhow::Result<GetAvailableSecretsResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let secrets = periphery_client(&server)?
      .request(api::GetSecrets {})
      .await
      .context("failed to get accounts from periphery")?;
    Ok(secrets)
  }
}
