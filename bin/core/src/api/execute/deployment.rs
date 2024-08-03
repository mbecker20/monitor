use anyhow::{anyhow, Context};
use formatting::format_serror;
use futures::future::join_all;
use monitor_client::{
  api::execute::*,
  entities::{
    build::{Build, ImageRegistry},
    config::core::AwsEcrConfig,
    deployment::{
      extract_registry_domain, Deployment, DeploymentActionState,
      DeploymentImage,
    },
    get_image_name,
    permission::PermissionLevel,
    server::{Server, ServerState},
    update::{Log, Update},
    user::User,
    Version,
  },
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  cloud::aws::ecr,
  config::core_config,
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    query::get_server_with_status, update::update_update,
  },
  monitor::update_cache_for_server,
  resource,
  state::{action_states, db_client, State},
};

use crate::helpers::update::init_execution_update;

async fn setup_deployment_execution(
  deployment: &str,
  user: &User,
  set_in_progress: impl Fn(&mut DeploymentActionState),
) -> anyhow::Result<(Deployment, Server)> {
  let deployment = resource::get_check_permissions::<Deployment>(
    deployment,
    user,
    PermissionLevel::Execute,
  )
  .await?;

  if deployment.config.server_id.is_empty() {
    return Err(anyhow!("deployment has no server configured"));
  }

  // get the action state for the deployment (or insert default).
  let action_state = action_states()
    .deployment
    .get_or_insert_default(&deployment.id)
    .await;

  // Will check to ensure deployment not already busy before updating, and return Err if so.
  // The returned guard will set the action state back to default when dropped.
  let _action_guard = action_state.update(set_in_progress)?;

  let (server, status) =
    get_server_with_status(&deployment.config.server_id).await?;
  if status != ServerState::Ok {
    return Err(anyhow!(
      "cannot send action when server is unreachable or disabled"
    ));
  }

  Ok((deployment, server))
}

impl Resolve<Deploy, (User, Update)> for State {
  #[instrument(name = "Deploy", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    Deploy {
      deployment,
      stop_signal,
      stop_time,
    }: Deploy,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (mut deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.deploying = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let (version, registry_token, aws_ecr) =
      match &deployment.config.image {
        DeploymentImage::Build { build_id, version } => {
          let build = resource::get::<Build>(build_id).await?;
          let image_name = get_image_name(&build, |label| {
            core_config()
              .aws_ecr_registries
              .iter()
              .find(|reg| &reg.label == label)
              .map(AwsEcrConfig::from)
          })
          .context("failed to create image name")?;
          let version = if version.is_none() {
            build.config.version
          } else {
            *version
          };
          // Remove ending patch if it is 0, this means use latest patch.
          let version_str = if version.patch == 0 {
            format!("{}.{}", version.major, version.minor)
          } else {
            version.to_string()
          };
          // Potentially add the build image_tag prefix
          let version_str = if build.config.image_tag.is_empty() {
            version_str
          } else {
            format!("{}-{version_str}", build.config.image_tag)
          };
          // replace image with corresponding build image.
          deployment.config.image = DeploymentImage::Image {
            image: format!("{image_name}:{version_str}"),
          };
          match build.config.image_registry {
            ImageRegistry::None(_) => (version, None, None),
            ImageRegistry::AwsEcr(label) => {
              let config = core_config()
                .aws_ecr_registries
                .iter()
                .find(|reg| reg.label == label)
                .with_context(|| {
                  format!(
                  "did not find config for aws ecr registry {label}"
                )
                })?;
              let token = ecr::get_ecr_token(
                &config.region,
                &config.access_key_id,
                &config.secret_access_key,
              )
              .await
              .context("failed to create aws ecr login token")?;
              (version, Some(token), Some(AwsEcrConfig::from(config)))
            }
            ImageRegistry::Standard(params) => {
              if deployment.config.image_registry_account.is_empty() {
                deployment.config.image_registry_account =
                  params.account
              }
              let token = core_config()
                .docker_registries
                .iter()
                .find(|registry| registry.domain == params.domain)
                .and_then(|provider| {
                  provider
                    .accounts
                    .iter()
                    .find(|account| {
                      account.username
                        == deployment.config.image_registry_account
                    })
                    .map(|account| account.token.clone())
                });
              (version, token, None)
            }
          }
        }
        DeploymentImage::Image { image } => {
          let domain = extract_registry_domain(image)?;
          let token =
            (!deployment.config.image_registry_account.is_empty())
              .then(|| {
                core_config()
                  .docker_registries
                  .iter()
                  .find(|registry| registry.domain == domain)
                  .and_then(|provider| {
                    provider
                      .accounts
                      .iter()
                      .find(|account| {
                        account.username
                          == deployment.config.image_registry_account
                      })
                      .map(|account| account.token.clone())
                  })
              })
              .flatten();
          (Version::default(), token, None)
        }
      };

    let secret_replacers = if !deployment.config.skip_secret_interp {
      interpolate_variables_secrets_into_environment(
        &mut deployment.config.environment,
        &mut update,
      )
      .await?
    } else {
      Default::default()
    };

    update.version = version;
    update_update(update.clone()).await?;

    match periphery
      .request(api::container::Deploy {
        deployment,
        stop_signal,
        stop_time,
        registry_token,
        aws_ecr,
        replacers: secret_replacers.into_iter().collect(),
      })
      .await
    {
      Ok(log) => update.logs.push(log),
      Err(e) => {
        update.push_error_log(
          "deploy container",
          format_serror(
            &e.context("failed to deploy container").into(),
          ),
        );
      }
    };

    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StartContainer, (User, Update)> for State {
  #[instrument(name = "StartContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartContainer { deployment }: StartContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.starting = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StartContainer {
        name: deployment.name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "start container",
        format_serror(&e.context("failed to start container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RestartContainer, (User, Update)> for State {
  #[instrument(name = "RestartContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartContainer { deployment }: RestartContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.restarting = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RestartContainer {
        name: deployment.name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "restart container",
        format_serror(&e.context("failed to restart container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PauseContainer, (User, Update)> for State {
  #[instrument(name = "PauseContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseContainer { deployment }: PauseContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.pausing = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::PauseContainer {
        name: deployment.name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "pause container",
        format_serror(&e.context("failed to pause container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<UnpauseContainer, (User, Update)> for State {
  #[instrument(name = "UnpauseContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseContainer { deployment }: UnpauseContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.unpausing = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::UnpauseContainer {
        name: deployment.name.clone(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "unpause container",
        format_serror(&e.context("failed to unpause container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopContainer, (User, Update)> for State {
  #[instrument(name = "StopContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopContainer {
      deployment,
      signal,
      time,
    }: StopContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.stopping = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StopContainer {
        name: deployment.name.clone(),
        signal: signal
          .unwrap_or(deployment.config.termination_signal)
          .into(),
        time: time
          .unwrap_or(deployment.config.termination_timeout)
          .into(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopAllContainers, (User, Update)> for State {
  #[instrument(name = "StopAllContainers", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopAllContainers { server }: StopAllContainers,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, status) = get_server_with_status(&server).await?;
    if status != ServerState::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    // get the action state for the server (or insert default).
    let action_state = action_states()
      .server
      .get_or_insert_default(&server.id)
      .await;

    // Will check to ensure server not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard = action_state
      .update(|state| state.stopping_containers = true)?;

    let deployments = find_collect(
      &db_client().await.deployments,
      doc! {
        "config.server_id": &server.id
      },
      None,
    )
    .await
    .context("failed to find deployments on server")?;

    let futures = deployments.iter().map(|deployment| async {
      let req = super::ExecuteRequest::StopContainer(StopContainer {
        deployment: deployment.id.clone(),
        signal: None,
        time: None,
      });
      (
        async {
          let update = init_execution_update(&req, &user).await?;
          State
            .resolve(
              StopContainer {
                deployment: deployment.id.clone(),
                signal: None,
                time: None,
              },
              (user.clone(), update),
            )
            .await
        }
        .await,
        deployment.name.clone(),
        deployment.id.clone(),
      )
    });
    let results = join_all(futures).await;
    let deployment_names = deployments
      .iter()
      .map(|d| format!("{} ({})", d.name, d.id))
      .collect::<Vec<_>>()
      .join("\n");
    update.push_simple_log("stopping containers", deployment_names);
    for (res, name, id) in results {
      if let Err(e) = res {
        update.push_error_log(
          "stop container failure",
          format_serror(
            &e.context(format!(
              "failed to stop container {name} ({id})"
            ))
            .into(),
          ),
        );
      }
    }

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RemoveContainer, (User, Update)> for State {
  #[instrument(name = "RemoveContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RemoveContainer {
      deployment,
      signal,
      time,
    }: RemoveContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user, |state| {
        state.removing = true
      })
      .await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RemoveContainer {
        name: deployment.name.clone(),
        signal: signal
          .unwrap_or(deployment.config.termination_signal)
          .into(),
        time: time
          .unwrap_or(deployment.config.termination_timeout)
          .into(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update.finalize();
    update_cache_for_server(&server).await;
    update_update(update.clone()).await?;

    Ok(update)
  }
}
