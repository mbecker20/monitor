use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::TerminationSignal, update::Update,
};

use super::MonitorExecuteRequest;

/// Deploys the container for the target deployment. Response: [Update].
///
/// 1. Pulls the image onto the target server.
/// 2. If the container is already running,
/// it will be stopped and removed using `docker container rm ${container_name}`.
/// 3. The container will be run using `docker run {...params}`,
/// where params are determined by the deployment's configuration.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct Deploy {
  /// Name or id
  pub deployment: String,
  /// Override the default termination signal specified in the deployment.
  pub stop_signal: Option<TerminationSignal>,
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
}

//

/// Starts the container for the target deployment. Response: [Update]
///
/// 1. Runs `docker start ${container_name}`.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StartContainer {
  /// Name or id
  pub deployment: String,
}

//

/// Stops the container for the target deployment. Response: [Update]
///
/// 1. Runs `docker stop ${container_name}`.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StopContainer {
  /// Name or id
  pub deployment: String,
  /// Override the default termination signal specified in the deployment.
  pub signal: Option<TerminationSignal>,
  /// Override the default termination max time.
  pub time: Option<i32>,
}

/// Stops all deployments on the target server. Response: [Update]
///
/// 1. Runs [StopContainer] on all deployments on the server concurrently.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StopAllContainers {
  /// Name or id
  pub server: String,
}

//

/// Stops and removes the container for the target deployment.
/// Reponse: [Update].
///
/// 1. The container is stopped and removed using `docker container rm ${container_name}`.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct RemoveContainer {
  /// Name or id.
  pub deployment: String,
  /// Override the default termination signal specified in the deployment.
  pub signal: Option<TerminationSignal>,
  /// Override the default termination max time.
  pub time: Option<i32>,
}
