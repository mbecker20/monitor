use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::ContainerSummary,
  server::{
    docker_image::ImageSummary,
    docker_network::DockerNetwork,
    stats::{
      SystemInformation, SystemProcess, SystemStats,
      SystemStatsRecord,
    },
    Server, ServerActionState, ServerListItem, ServerQuery,
    ServerState,
  },
  Timelength, I64,
};

use super::MonitorReadRequest;

//

/// Get a specific server. Response: [Server].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Server)]
pub struct GetServer {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetServerResponse = Server;

//

/// List servers matching optional query. Response: [ListServersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListServersResponse)]
pub struct ListServers {
  /// optional structured query to filter servers.
  #[serde(default)]
  pub query: ServerQuery,
}

#[typeshare]
pub type ListServersResponse = Vec<ServerListItem>;

//

/// List servers matching optional query. Response: [ListFullServersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullServersResponse)]
pub struct ListFullServers {
  /// optional structured query to filter servers.
  #[serde(default)]
  pub query: ServerQuery,
}

#[typeshare]
pub type ListFullServersResponse = Vec<Server>;

//

/// Get the state of the target server. Response: [GetServerStateResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServerStateResponse)]
pub struct GetServerState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

/// The response for [GetServerState].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerStateResponse {
  /// The server status.
  pub status: ServerState,
}

//

/// Get current action state for the servers. Response: [ServerActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ServerActionState)]
pub struct GetServerActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetServerActionStateResponse = ServerActionState;

//

/// Get the version of the monitor periphery agent on the target server.
/// Response: [GetPeripheryVersionResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetPeripheryVersionResponse)]
pub struct GetPeripheryVersion {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

/// Response for [GetPeripheryVersion].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPeripheryVersionResponse {
  /// The version of periphery.
  pub version: String,
}

//

/// Get the docker networks on the server. Response: [GetDockerNetworksResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerNetworksResponse)]
pub struct GetDockerNetworks {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetDockerNetworksResponse = Vec<DockerNetwork>;

//

/// Get the docker images locally cached on the target server.
/// Response: [GetDockerImagesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerImagesResponse)]
pub struct GetDockerImages {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetDockerImagesResponse = Vec<ImageSummary>;

//

/// Get all docker containers on the target server.
/// Response: [GetDockerContainersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerContainersResponse)]
pub struct GetDockerContainers {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetDockerContainersResponse = Vec<ContainerSummary>;

//

/// Get the system information of the target server.
/// Response: [SystemInformation].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemInformationResponse)]
pub struct GetSystemInformation {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetSystemInformationResponse = SystemInformation;

//

/// Get the system stats on the target server. Response: [SystemStats].
///
/// Note. This does not hit the server directly. The stats come from an
/// in memory cache on the core, which hits the server periodically
/// to keep it up to date.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemStatsResponse)]
pub struct GetSystemStats {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetSystemStatsResponse = SystemStats;

//

/// Get the processes running on the target server.
/// Response: [GetSystemProcessesResponse].
///
/// Note. This does not hit the server directly. The procedures come from an
/// in memory cache on the core, which hits the server periodically
/// to keep it up to date.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemProcessesResponse)]
pub struct GetSystemProcesses {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetSystemProcessesResponse = Vec<SystemProcess>;

//

/// Paginated endpoint serving historical (timeseries) server stats for graphing.
/// Response: [GetHistoricalServerStatsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetHistoricalServerStatsResponse)]
pub struct GetHistoricalServerStats {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The granularity of the data.
  pub granularity: Timelength,
  /// Page of historical data. Default is 0, which is the most recent data.
  /// Use with the `next_page` field of the response.
  #[serde(default)]
  pub page: u32,
}

/// Response to [GetHistoricalServerStats].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHistoricalServerStatsResponse {
  /// The timeseries page of data.
  pub stats: Vec<SystemStatsRecord>,
  /// If there is a next page of data, pass this to `page` to get it.
  pub next_page: Option<u32>,
}

//

/// Gets a summary of data relating to all servers.
/// Response: [GetServersSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServersSummaryResponse)]
pub struct GetServersSummary {}

/// Response for [GetServersSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetServersSummaryResponse {
  /// The total number of servers.
  pub total: I64,
  /// The number of healthy (`status: OK`) servers.
  pub healthy: I64,
  /// The number of unhealthy servers.
  pub unhealthy: I64,
  /// The number of disabled servers.
  pub disabled: I64,
}
