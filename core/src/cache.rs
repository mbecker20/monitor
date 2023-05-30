use async_timing_util::{unix_timestamp_ms, wait_until_timelength, Timelength};
use futures_util::future::join_all;
use mungos::mongodb::bson::doc;
use types::{BasicContainerInfo, DockerContainerState, Server, ServerStatus};

use crate::state::State;

#[derive(Default)]
pub struct CachedDeploymentStatus {
    pub id: String,
    pub state: DockerContainerState,
    pub container: Option<BasicContainerInfo>,
}

pub struct CachedServerStatus {
    pub id: String,
    pub status: ServerStatus,
}

impl State {
    pub async fn manage_status_cache(&self) {
        loop {
            wait_until_timelength(Timelength::FifteenSeconds, 1000).await;
            let servers = self
                .db
                .servers
                .get_some(doc! { "enabled": true }, None)
                .await;
            if let Err(e) = &servers {
                eprintln!(
                    "{} | failed to get server list (manage status cache) | {e:#?}",
                    unix_timestamp_ms()
                )
            }
            let servers = servers.unwrap();
            let futures = servers
                .into_iter()
                .map(|server| self.update_deployment_status_cache(server));
            join_all(futures).await;
        }
    }

    async fn update_deployment_status_cache(&self, server: Server) {
        let deployments = self
            .db
            .deployments
            .get_some(doc! { "server_id": &server.id }, None)
            .await;
        if let Err(e) = &deployments {
            eprintln!("{} | failed to get deployments list from mongo (manage status cache) | server id: {} | {e:#?}", server.id, unix_timestamp_ms());
            return;
        }
        let deployments = deployments.unwrap();
        let containers = self.periphery.container_list(&server).await;
        if containers.is_err() {
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
            return;
        }
        let containers = containers.unwrap();
        for deployment in deployments {
            let container = containers
                .iter()
                .find(|c| c.name == deployment.name)
                .map(|c| c.to_owned());
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
}
