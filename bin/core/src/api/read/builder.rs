use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{self, *},
  entities::{
    builder::{Builder, BuilderConfig, BuilderListItem},
    PermissionLevel,
  },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<GetBuilder, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuilder { id }: GetBuilder,
    user: RequestUser,
  ) -> anyhow::Result<Builder> {
    self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await
  }
}

#[async_trait]
impl Resolve<ListBuilders, RequestUser> for State {
  async fn resolve(
    &self,
    ListBuilders { query }: ListBuilders,
    user: RequestUser,
  ) -> anyhow::Result<Vec<BuilderListItem>> {
    <State as StateResource<Builder>>::list_resources_for_user(
      self, query, &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<GetBuildersSummary, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuildersSummary {}: GetBuildersSummary,
    user: RequestUser,
  ) -> anyhow::Result<GetBuildersSummaryResponse> {
    let query = if user.is_admin {
      None
    } else {
      let query = doc! {
          format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
      };
      Some(query)
    };
    let total = self
      .db
      .builders
      .count_documents(query, None)
      .await
      .context("failed to count all builder documents")?;
    let res = GetBuildersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetBuilderAvailableAccounts, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuilderAvailableAccounts { id }: GetBuilderAvailableAccounts,
    user: RequestUser,
  ) -> anyhow::Result<GetBuilderAvailableAccountsResponse> {
    let builder: Builder = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await?;
    match builder.config {
      BuilderConfig::Aws(config) => {
        Ok(GetBuilderAvailableAccountsResponse {
          github: config.github_accounts,
          docker: config.docker_accounts,
        })
      }
      BuilderConfig::Server(config) => {
        let res = self
          .resolve(
            read::GetAvailableAccounts {
              server_id: config.id,
            },
            user,
          )
          .await?;
        Ok(GetBuilderAvailableAccountsResponse {
          github: res.github,
          docker: res.docker,
        })
      }
    }
  }
}