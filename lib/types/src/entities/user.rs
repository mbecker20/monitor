use bson::serde_helpers::hex_string_as_object_id;
use mungos::MungosIndexed;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{I64, i64_is_zero};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, MungosIndexed)]
pub struct User {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,

    #[unique_index]
    pub username: String,

    #[serde(default)]
    #[index]
    pub enabled: bool,

    #[serde(default)]
    pub admin: bool,

    #[serde(default)]
    pub create_server_permissions: bool,

    #[serde(default)]
    pub create_build_permissions: bool,

    pub avatar: Option<String>,

    #[serde(default)]
    pub secrets: Vec<ApiSecret>,

    pub password: Option<String>,

    pub github_id: Option<String>,

    pub google_id: Option<String>,

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    pub created_at: I64,

    #[serde(default)]
    pub updated_at: I64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ApiSecret {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hash: String,
    pub created_at: I64,
    pub expires: Option<I64>,
}