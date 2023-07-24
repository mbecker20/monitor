use std::time::Instant;

use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::requests::execute::*;
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUser, RequestUserExtension},
    state::{State, StateExtension},
};

mod build;
mod deployment;
mod repo;
mod server;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
enum ExecuteRequest {
    // ==== SERVER ====
    PruneContainers(PruneDockerContainers),
    PruneImages(PruneDockerImages),
    PruneNetworks(PruneDockerNetworks),

    // ==== DEPLOYMENT ====
    Deploy(Deploy),
    StartContainer(StartContainer),
    StopContainer(StopContainer),
    RemoveContainer(RemoveContainer),

    // ==== BUILD ====
    RunBuild(RunBuild),

    // ==== REPO ====
    CloneRepo(CloneRepo),
    PullRepo(PullRepo),
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ExecuteRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!(
                        "/execute request {req_id} | user: {} ({}) | {request:?}",
                        user.username, user.id
                    );
                    let res = tokio::spawn(async move {
                        state
                            .resolve_request(request, user)
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
                    if let Err(e) = &res {
                        info!("/execute request {req_id} SPAWN ERROR: {e:#?}");
                    }
                    let res = res?;
                    if let Err(e) = &res {
                        info!("/execute request {req_id} ERROR: {e:#?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/execute request {req_id} | resolve time: {elapsed:?}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}