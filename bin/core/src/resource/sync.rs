use std::time::Duration;

use anyhow::Context;
use mongo_indexed::doc;
use monitor_client::entities::{
  monitor_timestamp,
  resource::Resource,
  sync::{
    PartialResourceSyncConfig, PendingSyncUpdatesData, ResourceSync,
    ResourceSyncConfig, ResourceSyncConfigDiff, ResourceSyncInfo,
    ResourceSyncListItem, ResourceSyncListItemInfo,
    ResourceSyncQuerySpecifics, ResourceSyncState,
  },
  update::Update,
  user::User,
  Operation, ResourceTargetVariant,
};
use mungos::{
  find::find_collect,
  mongodb::{options::FindOneOptions, Collection},
};

use crate::state::{
  action_states, db_client, resource_sync_state_cache,
};

impl super::MonitorResource for ResourceSync {
  type Config = ResourceSyncConfig;
  type PartialConfig = PartialResourceSyncConfig;
  type ConfigDiff = ResourceSyncConfigDiff;
  type Info = ResourceSyncInfo;
  type ListItem = ResourceSyncListItem;
  type QuerySpecifics = ResourceSyncQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::ResourceSync
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.resource_syncs
  }

  async fn to_list_item(
    resource_sync: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let state = get_resource_sync_state(
      &resource_sync.id,
      &resource_sync.info.pending.data,
    )
    .await;
    ResourceSyncListItem {
      id: resource_sync.id,
      name: resource_sync.name,
      tags: resource_sync.tags,
      resource_type: ResourceTargetVariant::ResourceSync,
      info: ResourceSyncListItemInfo {
        git_provider: resource_sync.config.git_provider,
        repo: resource_sync.config.repo,
        branch: resource_sync.config.branch,
        last_sync_ts: resource_sync.info.last_sync_ts,
        last_sync_hash: resource_sync.info.last_sync_hash,
        last_sync_message: resource_sync.info.last_sync_message,
        state,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .resource_sync
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateResourceSync
  }

  fn user_can_create(user: &User) -> bool {
    user.admin
  }

  async fn validate_create_config(
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateResourceSync
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    _updated: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteResourceSync
  }

  async fn pre_delete(
    resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    db_client().await.alerts
      .update_many(
        doc! { "target.type": "ResourceSync", "target.id": &resource.id },
        doc! { "$set": {
          "resolved": true,
          "resolved_ts": monitor_timestamp()
        } },
      )
      .await
      .context("failed to close deleted sync alerts")?;

    Ok(())
  }

  async fn post_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}

pub fn spawn_resource_sync_state_refresh_loop() {
  tokio::spawn(async move {
    loop {
      refresh_resource_sync_state_cache().await;
      tokio::time::sleep(Duration::from_secs(60)).await;
    }
  });
}

pub async fn refresh_resource_sync_state_cache() {
  let _ = async {
    let resource_syncs =
      find_collect(&db_client().await.resource_syncs, None, None)
        .await
        .context("failed to get resource_syncs from db")?;
    let cache = resource_sync_state_cache();
    for resource_sync in resource_syncs {
      let state =
        get_resource_sync_state_from_db(&resource_sync.id).await;
      cache.insert(resource_sync.id, state).await;
    }
    anyhow::Ok(())
  }
  .await
  .inspect_err(|e| {
    error!("failed to refresh resource_sync state cache | {e:#}")
  });
}

async fn get_resource_sync_state(
  id: &String,
  data: &PendingSyncUpdatesData,
) -> ResourceSyncState {
  if let Some(state) = action_states()
    .resource_sync
    .get(id)
    .await
    .and_then(|s| {
      s.get()
        .map(|s| {
          if s.syncing {
            Some(ResourceSyncState::Syncing)
          } else {
            None
          }
        })
        .ok()
    })
    .flatten()
  {
    return state;
  }
  let data = match data {
    PendingSyncUpdatesData::Err(_) => {
      return ResourceSyncState::Failed
    }
    PendingSyncUpdatesData::Ok(data) => data,
  };
  if !data.no_updates() {
    return ResourceSyncState::Pending;
  }
  resource_sync_state_cache()
    .get(id)
    .await
    .unwrap_or_default()
}

async fn get_resource_sync_state_from_db(
  id: &str,
) -> ResourceSyncState {
  async {
    let state = db_client()
      .await
      .updates
      .find_one(doc! {
        "target.type": "ResourceSync",
        "target.id": id,
        "operation": "RunSync"
      })
      .with_options(
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          ResourceSyncState::Ok
        } else {
          ResourceSyncState::Failed
        }
      })
      .unwrap_or(ResourceSyncState::Ok);
    anyhow::Ok(state)
  }
  .await
  .inspect_err(|e| {
    warn!(
      "failed to get resource sync state from db for {id} | {e:#}"
    )
  })
  .unwrap_or(ResourceSyncState::Unknown)
}
