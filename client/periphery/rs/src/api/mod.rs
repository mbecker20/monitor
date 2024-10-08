use komodo_client::entities::{
  config::{DockerRegistry, GitProvider},
  docker::{
    container::ContainerListItem, image::ImageListItem,
    network::NetworkListItem, volume::VolumeListItem,
  },
  stack::ComposeProject,
  update::Log,
  SystemCommand,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use serror::Serror;

pub mod build;
pub mod compose;
pub mod container;
pub mod git;
pub mod image;
pub mod network;
pub mod stats;
pub mod volume;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetHealthResponse)]
pub struct GetHealth {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
  pub version: String,
}

/// Returns all containers, networks, images, compose projects
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDockerListsResponse)]
pub struct GetDockerLists {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDockerListsResponse {
  pub containers: Result<Vec<ContainerListItem>, Serror>,
  pub networks: Result<Vec<NetworkListItem>, Serror>,
  pub images: Result<Vec<ImageListItem>, Serror>,
  pub volumes: Result<Vec<VolumeListItem>, Serror>,
  pub projects: Result<Vec<ComposeProject>, Serror>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ListGitProvidersResponse)]
pub struct ListGitProviders {}

pub type ListGitProvidersResponse = Vec<GitProvider>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ListDockerRegistriesResponse)]
pub struct ListDockerRegistries {}

pub type ListDockerRegistriesResponse = Vec<DockerRegistry>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<String>)]
pub struct ListSecrets {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneSystem {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RunCommand {
  pub command: SystemCommand,
}
