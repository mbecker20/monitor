use anyhow::anyhow;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use monitor_types::entities::{
    update::{ResourceTarget, ResourceTargetVariant},
    user::User,
    PermissionLevel,
};
use serde_json::json;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
    auth::RequestUser,
    state::{State, StateExtension},
};

pub fn router() -> Router {
    Router::new().route("/update", get(ws_handler))
}

async fn ws_handler(state: StateExtension, ws: WebSocketUpgrade) -> impl IntoResponse {
    let mut receiver = state.update.receiver.resubscribe();
    ws.on_upgrade(|socket| async move {
        let login_res = state.ws_login(socket).await;
        if login_res.is_none() {
            return;
        }
        let (socket, user) = login_res.unwrap();
        let (mut ws_sender, mut ws_reciever) = socket.split();
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            loop {
                let update = select! {
                    _ = cancel_clone.cancelled() => break,
                    update = receiver.recv() => {update.expect("failed to recv update msg")}
                };
                let user = state.db.users.find_one_by_id(&user.id).await;
                if user.is_err()
                    || user.as_ref().unwrap().is_none()
                    || !user.as_ref().unwrap().as_ref().unwrap().enabled
                {
                    let _ = ws_sender
                        .send(Message::Text(json!({ "type": "INVALID_USER" }).to_string()))
                        .await;
                    let _ = ws_sender.close().await;
                    return;
                }
                let user = user.unwrap().unwrap(); // already handle cases where this panics in the above early return
                let res = state
                    .user_can_see_update(&user, &user.id, &update.target)
                    .await;
                if let Err(_e) = res {
                    // handle
                    return;
                } else {
                    let _ = ws_sender
                        .send(Message::Text(serde_json::to_string(&update).unwrap()))
                        .await;
                }
            }
        });

        while let Some(msg) = ws_reciever.next().await {
            match msg {
                Ok(msg) => {
                    if let Message::Close(_) = msg {
                        cancel.cancel();
                        return;
                    }
                }
                Err(_) => {
                    cancel.cancel();
                    return;
                }
            }
        }
    })
}

impl State {
    pub async fn ws_login(&self, mut socket: WebSocket) -> Option<(WebSocket, RequestUser)> {
        if let Some(jwt) = socket.recv().await {
            match jwt {
                Ok(jwt) => match jwt {
                    Message::Text(jwt) => match self.auth_jwt_check_enabled(&jwt).await {
                        Ok(user) => {
                            let _ = socket.send(Message::Text("LOGGED_IN".to_string())).await;
                            Some((socket, user))
                        }
                        Err(e) => {
                            let _ = socket
                                .send(Message::Text(format!(
                                    "failed to authenticate user | {e:#?}"
                                )))
                                .await;
                            let _ = socket.close().await;
                            None
                        }
                    },
                    msg => {
                        let _ = socket
                            .send(Message::Text(format!("invalid login msg: {msg:#?}")))
                            .await;
                        let _ = socket.close().await;
                        None
                    }
                },
                Err(e) => {
                    let _ = socket
                        .send(Message::Text(format!("failed to get jwt message: {e:#?}")))
                        .await;
                    let _ = socket.close().await;
                    None
                }
            }
        } else {
            let _ = socket
                .send(Message::Text(String::from("failed to get jwt message")))
                .await;
            let _ = socket.close().await;
            None
        }
    }

    async fn user_can_see_update(
        &self,
        user: &User,
        user_id: &str,
        update_target: &ResourceTarget,
    ) -> anyhow::Result<()> {
        if user.admin {
            return Ok(());
        }
        let (permissions, target) = match update_target {
            ResourceTarget::Server(server_id) => {
                let permissions = self
                    .get_user_permission_on_server(user_id, server_id)
                    .await?;
                (permissions, ResourceTargetVariant::Server)
            }
            ResourceTarget::Deployment(deployment_id) => {
                let permissions = self
                    .get_user_permission_on_deployment(user_id, deployment_id)
                    .await?;
                (permissions, ResourceTargetVariant::Deployment)
            }
            ResourceTarget::Build(build_id) => {
                let permissions = self.get_user_permission_on_build(user_id, build_id).await?;
                (permissions, ResourceTargetVariant::Build)
            }
            ResourceTarget::Builder(builder_id) => {
                let permissions = self
                    .get_user_permission_on_builder(user_id, builder_id)
                    .await?;
                (permissions, ResourceTargetVariant::Builder)
            }
            ResourceTarget::Repo(repo_id) => {
                let permissions = self.get_user_permission_on_repo(user_id, repo_id).await?;
                (permissions, ResourceTargetVariant::Repo)
            }
            ResourceTarget::Alerter(alerter_id) => {
                let permissions = self
                    .get_user_permission_on_alerter(user_id, alerter_id)
                    .await?;
                (permissions, ResourceTargetVariant::Alerter)
            }
            ResourceTarget::System => {
                return Err(anyhow!("user not admin, can't recieve system updates"))
            }
        };
        if permissions != PermissionLevel::None {
            Ok(())
        } else {
            Err(anyhow!("user does not have permissions on {target}"))
        }
    }
}