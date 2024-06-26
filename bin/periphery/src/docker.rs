use std::sync::OnceLock;

use anyhow::Context;
use bollard::{container::ListContainersOptions, Docker};
use monitor_client::entities::{
  deployment::ContainerSummary,
  server::{
    docker_image::ImageSummary, docker_network::DockerNetwork,
  },
};

pub fn docker_client() -> &'static DockerClient {
  static DOCKER_CLIENT: OnceLock<DockerClient> = OnceLock::new();
  DOCKER_CLIENT.get_or_init(Default::default)
}

pub struct DockerClient {
  docker: Docker,
}

impl Default for DockerClient {
  fn default() -> DockerClient {
    DockerClient {
      docker: Docker::connect_with_local_defaults()
        .expect("failed to connect to docker daemon"),
    }
  }
}

impl DockerClient {
  pub async fn list_containers(
    &self,
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    let res = self
      .docker
      .list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
      }))
      .await?
      .into_iter()
      .map(|s| {
        let info = ContainerSummary {
          id: s.id.unwrap_or_default(),
          name: s
            .names
            .context("no names on container")?
            .pop()
            .context("no names on container (empty vec)")?
            .replace('/', ""),
          image: s.image.unwrap_or(String::from("unknown")),
          state: s
            .state
            .context("no container state")?
            .parse()
            .context("failed to parse container state")?,
          status: s.status,
          labels: s.labels.unwrap_or_default(),
        };
        Ok::<_, anyhow::Error>(info)
      })
      .collect::<anyhow::Result<Vec<ContainerSummary>>>()?;
    Ok(res)
  }

  pub async fn list_networks(
    &self,
  ) -> anyhow::Result<Vec<DockerNetwork>> {
    let networks = self
      .docker
      .list_networks::<String>(None)
      .await?
      .into_iter()
      .map(|network| network.into())
      .collect();
    Ok(networks)
  }

  pub async fn list_images(
    &self,
  ) -> anyhow::Result<Vec<ImageSummary>> {
    let images = self
      .docker
      .list_images::<String>(None)
      .await?
      .into_iter()
      .map(|i| i.into())
      .collect();
    Ok(images)
  }
}
