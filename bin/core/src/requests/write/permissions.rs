use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
  entities::{
    alerter::Alerter,
    repo::Repo,
    server::Server,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    Operation,
  },
  monitor_timestamp,
  requests::write::{
    UpdateUserPermissions, UpdateUserPermissionsOnTarget,
  },
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  mongodb::bson::{doc, Document},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<UpdateUserPermissions, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateUserPermissions {
      user_id,
      enabled,
      create_servers,
      create_builds,
    }: UpdateUserPermissions,
    admin: RequestUser,
  ) -> anyhow::Result<Update> {
    let start_ts = monitor_timestamp();
    if !admin.is_admin {
      return Err(anyhow!("this method is admin only"));
    }

    let user = find_one_by_id(&self.db.users, &user_id)
      .await
      .context("failed to query mongo for user")?
      .context("did not find user with given id")?;
    if user.admin {
      return Err(anyhow!(
        "cannot use this method to update other admins permissions"
      ));
    }
    let mut update_doc = Document::new();
    if let Some(enabled) = enabled {
      update_doc.insert("enabled", enabled);
    }
    if let Some(create_servers) = create_servers {
      update_doc.insert("create_server_permissions", create_servers);
    }
    if let Some(create_builds) = create_builds {
      update_doc.insert("create_build_permissions", create_builds);
    }

    update_one_by_id(
      &self.db.users,
      &user_id,
      mungos::update::Update::Set(update_doc),
      None,
    )
    .await?;
    let end_ts = monitor_timestamp();
    let mut update = Update {
      target: ResourceTarget::System("system".to_string()),
      operation: Operation::UpdateUserPermissions,
      logs: vec![Log::simple(
        "modify user enabled",
        format!(
          "update permissions for {} ({})\nenabled: {enabled:?}\ncreate servers: {create_servers:?}\ncreate builds: {create_builds:?}", 
          user.username,
          user.id,

        ),
      )],
      start_ts,
      end_ts: end_ts.into(),
      status: UpdateStatus::Complete,
      success: true,
      operator: admin.id.clone(),
      ..Default::default()
    };
    update.id = self.add_update(update.clone()).await?;
    Ok(update)
  }
}

#[async_trait]
impl Resolve<UpdateUserPermissionsOnTarget, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateUserPermissionsOnTarget {
      user_id,
      permission,
      target,
    }: UpdateUserPermissionsOnTarget,
    admin: RequestUser,
  ) -> anyhow::Result<Update> {
    let start_ts = monitor_timestamp();
    if !admin.is_admin {
      return Err(anyhow!("this method is admin only"));
    }

    let user = self.get_user(&user_id).await?;
    if user.admin {
      return Err(anyhow!(
        "cannot use this method to update other admins permissions"
      ));
    }
    if !user.enabled {
      return Err(anyhow!("user not enabled"));
    }
    let log_text = match &target {
      ResourceTarget::System(_) => {
        return Err(anyhow!("target can not be system"))
      }
      ResourceTarget::Build(id) => {
        let build = find_one_by_id(&self.db.builds, id)
          .await
          .context("failed at find build query")?
          .ok_or(anyhow!("failed to find a build with id {id}"))?;

        update_one_by_id(
          &self.db.builds,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on build {}",
          user.username, permission, build.name
        )
      }
      ResourceTarget::Builder(id) => {
        let builder = find_one_by_id(&self.db.builders, id)
          .await
          .context("failed at find builder query")?
          .with_context(|| {
            format!("failed to find a builder with id {id}")
          })?;

        update_one_by_id(
          &self.db.builders,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on builder {}",
          user.username, permission, builder.name
        )
      }
      ResourceTarget::Deployment(id) => {
        let deployment = find_one_by_id(&self.db.deployments, id)
          .await
          .context("failed at find deployment query")?
          .with_context(|| {
            format!("failed to find a deployment with id {id}")
          })?;

        update_one_by_id(
          &self.db.deployments,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on deployment {}",
          user.username, permission, deployment.name
        )
      }
      ResourceTarget::Server(id) => {
        // find_one_by_id(&self.db.servers, id)
        let server: Server = self.get_resource(id).await?;

        update_one_by_id(
          &self.db.servers,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on server {}",
          user.username, permission, server.name
        )
      }
      ResourceTarget::Repo(id) => {
        let repo: Repo = self.get_resource(id).await?;

        update_one_by_id(
          &self.db.repos,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on repo {}",
          user.username, permission, repo.name
        )
      }
      ResourceTarget::Alerter(id) => {
        let alerter: Alerter = self.get_resource(id).await?;
        update_one_by_id(
          &self.db.alerters,
          id,
          mungos::update::Update::Set(doc! {
            format!("permissions.{}", user_id): permission.to_string()
          }),
          None,
        )
        .await?;
        format!(
          "user {} given {} permissions on alerter {}",
          user.username, permission, alerter.name
        )
      }
    };
    let mut update = Update {
      operation: Operation::UpdateUserPermissionsOnTarget,
      start_ts,
      success: true,
      operator: admin.id.clone(),
      status: UpdateStatus::Complete,
      target: target.clone(),
      logs: vec![Log::simple("modify permissions", log_text)],
      end_ts: monitor_timestamp().into(),
      ..Default::default()
    };
    update.id = self.add_update(update.clone()).await?;
    Ok(update)
  }
}