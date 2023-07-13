use std::time::Instant;

use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::requests::read::*;
use resolver_api::{derive::Resolver, Resolve, ResolveToString, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUser, RequestUserExtension},
    state::{State, StateExtension},
};

mod build;
mod builder;
mod deployment;
mod repo;
mod search;
mod server;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
enum ReadRequest {
    // ==== SEARCH ====
    FindResources(FindResources),

    // ==== SERVER ====
    GetPeripheryVersion(GetPeripheryVersion),
    GetSystemInformation(GetSystemInformation),
    GetDockerContainers(GetDockerContainers),
    GetDockerImages(GetDockerImages),
    GetDockerNetworks(GetDockerNetworks),
    GetServer(GetServer),
    ListServers(ListServers),
    GetServerActionState(GetServerActionState),
    // STATS
    #[to_string_resolver]
    GetAllSystemStats(GetAllSystemStats),
    #[to_string_resolver]
    GetBasicSystemStats(GetBasicSystemStats),
    #[to_string_resolver]
    GetCpuUsage(GetCpuUsage),
    #[to_string_resolver]
    GetDiskUsage(GetDiskUsage),
    #[to_string_resolver]
    GetNetworkUsage(GetNetworkUsage),
    #[to_string_resolver]
    GetSystemProcesses(GetSystemProcesses),
    #[to_string_resolver]
    GetSystemComponents(GetSystemComponents),

    // ==== DEPLOYMENT ====
    GetDeployment(GetDeployment),
    ListDeployments(ListDeployments),
    GetDeploymentActionState(GetDeploymentActionState),

    // ==== BUILD ====
    GetBuild(GetBuild),
    ListBuilds(ListBuilds),
    GetBuildActionState(GetBuildActionState),

    // ==== BUILDER ====
    GetBuilder(GetBuilder),
    ListBuilders(ListBuilders),

    // ==== REPO ====
    GetRepo(GetRepo),
    ListRepos(ListRepos),
    GetRepoActionState(GetRepoActionState),
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ReadRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!(
                        "/read request {req_id} | user: {} ({}) | {request:?}",
                        user.username, user.id
                    );
                    let res = state
                        .resolve_request(request, user)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
                    if let Err(e) = &res {
                        info!("/read request {req_id} ERROR: {e:#?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/read request {req_id} | resolve time: {elapsed:?}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}
