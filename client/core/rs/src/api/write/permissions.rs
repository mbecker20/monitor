use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  permission::{PermissionLevel, UserTarget},
  NoData, ResourceTarget, ResourceTargetVariant,
};

use super::KomodoWriteRequest;

/// **Admin only.** Update a user or user groups permission on a resource.
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoWriteRequest)]
#[response(UpdatePermissionOnTargetResponse)]
pub struct UpdatePermissionOnTarget {
  /// Specify the user or user group.
  pub user_target: UserTarget,
  /// Specify the target resource.
  pub resource_target: ResourceTarget,
  /// Specify the permission level.
  pub permission: PermissionLevel,
}

#[typeshare]
pub type UpdatePermissionOnTargetResponse = NoData;

//

/// **Admin only.** Update a user or user groups base permission level on a resource type.
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoWriteRequest)]
#[response(UpdatePermissionOnResourceTypeResponse)]
pub struct UpdatePermissionOnResourceType {
  /// Specify the user or user group.
  pub user_target: UserTarget,
  /// The resource type: eg. Server, Build, Deployment, etc.
  pub resource_type: ResourceTargetVariant,
  /// The base permission level.
  pub permission: PermissionLevel,
}

#[typeshare]
pub type UpdatePermissionOnResourceTypeResponse = NoData;

//

/// **Admin only.** Update a user's "base" permissions, eg. "enabled".
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoWriteRequest)]
#[response(UpdateUserBasePermissionsResponse)]
pub struct UpdateUserBasePermissions {
  /// The target user.
  pub user_id: String,
  /// If specified, will update users enabled state.
  pub enabled: Option<bool>,
  /// If specified, will update user's ability to create servers.
  pub create_servers: Option<bool>,
  /// If specified, will update user's ability to create builds.
  pub create_builds: Option<bool>,
}

#[typeshare]
pub type UpdateUserBasePermissionsResponse = NoData;

/// **Super Admin only.** Update's whether a user is admin.
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoWriteRequest)]
#[response(UpdateUserAdminResponse)]
pub struct UpdateUserAdmin {
  /// The target user.
  pub user_id: String,
  /// Whether user should be admin.
  pub admin: bool,
}

#[typeshare]
pub type UpdateUserAdminResponse = NoData;
