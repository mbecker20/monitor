use async_timing_util::{unix_timestamp_ms, wait_until_timelength, Timelength};
use futures_util::future::join_all;
use monitor_types::{
    BasicContainerInfo, Deployment, DockerContainerState, Server, ServerStatus, SystemStats,
    SystemStatsQuery,
};
use mungos::mongodb::bson::doc;

use crate::state::State;

#[derive(Default)]
pub struct CachedDeploymentStatus {
    pub id: String,
    pub state: DockerContainerState,
    pub container: Option<BasicContainerInfo>,
}

#[derive(Default)]
pub struct CachedServerStatus {
    pub server: Server,
    pub status: ServerStatus,
    pub version: String,
    pub stats: Option<SystemStats>,
}

impl State {
    pub async fn manage_status_cache(&self) {
        loop {
            wait_until_timelength(Timelength::FiveSeconds, 500).await;
            let servers = self.db.servers.get_some(None, None).await;
            if let Err(e) = &servers {
                eprintln!(
                    "{} | failed to get server list (manage status cache) | {e:#?}",
                    unix_timestamp_ms()
                )
            }
            let servers = servers.unwrap();
            let futures = servers
                .into_iter()
                .map(|server| async move { self.update_status_cache(&server).await });
            join_all(futures).await;
        }
    }

    pub async fn update_status_cache(&self, server: &Server) {
        let deployments = self
            .db
            .deployments
            .get_some(doc! { "server_id": &server.id }, None)
            .await;
        if let Err(e) = &deployments {
            eprintln!("{} | failed to get deployments list from mongo (update status cache) | server id: {} | {e:#?}", server.id, unix_timestamp_ms());
            return;
        }
        let deployments = deployments.unwrap();
        if !server.enabled {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(
                server,
                ServerStatus::Disabled,
                String::from("unknown"),
                None,
            )
            .await;
            return;
        }
        let version = self.periphery.get_version(server).await;
        if version.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            return;
        }
        let stats = self
            .periphery
            .get_system_stats(server, &SystemStatsQuery::all())
            .await;
        if stats.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            return;
        }
        self.insert_server_status(
            server,
            ServerStatus::Ok,
            version.unwrap(),
            stats.unwrap().into(),
        )
        .await;
        let containers = self.periphery.container_list(server).await;
        if containers.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            return;
        }
        let containers = containers.unwrap();
        for deployment in deployments {
            let container = containers
                .iter()
                .find(|c| c.name == deployment.name)
                .cloned();
            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    CachedDeploymentStatus {
                        id: deployment.id,
                        state: container
                            .as_ref()
                            .map(|c| c.state)
                            .unwrap_or(DockerContainerState::NotDeployed),
                        container,
                    }
                    .into(),
                )
                .await;
        }
    }

    async fn insert_deployments_status_unknown(&self, deployments: Vec<Deployment>) {
        for deployment in deployments {
            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    CachedDeploymentStatus {
                        id: deployment.id,
                        state: DockerContainerState::Unknown,
                        container: None,
                    }
                    .into(),
                )
                .await;
        }
    }

    async fn insert_server_status(
        &self,
        server: &Server,
        status: ServerStatus,
        version: String,
        stats: Option<SystemStats>,
    ) {
        self.server_status_cache
            .insert(
                server.id.clone(),
                CachedServerStatus {
                    server: server.clone(),
                    status,
                    version,
                    stats,
                }
                .into(),
            )
            .await;
    }
}
