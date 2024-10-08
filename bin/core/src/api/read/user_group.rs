use std::str::FromStr;

use anyhow::Context;
use komodo_client::{
  api::read::{
    GetUserGroup, GetUserGroupResponse, ListUserGroups,
    ListUserGroupsResponse,
  },
  entities::user::User,
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOptions,
  },
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

impl Resolve<GetUserGroup, User> for State {
  async fn resolve(
    &self,
    GetUserGroup { user_group }: GetUserGroup,
    user: User,
  ) -> anyhow::Result<GetUserGroupResponse> {
    let mut filter = match ObjectId::from_str(&user_group) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": &user_group },
    };
    // Don't allow non admin users to get UserGroups they aren't a part of.
    if !user.admin {
      // Filter for only UserGroups which contain the users id
      filter.insert("users", &user.id);
    }
    db_client()
      .user_groups
      .find_one(filter)
      .await
      .context("failed to query db for user groups")?
      .context("no UserGroup found with given name or id")
  }
}

impl Resolve<ListUserGroups, User> for State {
  async fn resolve(
    &self,
    ListUserGroups {}: ListUserGroups,
    user: User,
  ) -> anyhow::Result<ListUserGroupsResponse> {
    let mut filter = Document::new();
    if !user.admin {
      filter.insert("users", &user.id);
    }
    find_collect(
      &db_client().user_groups,
      filter,
      FindOptions::builder().sort(doc! { "name": 1 }).build(),
    )
    .await
    .context("failed to query db for UserGroups")
  }
}
