use std::path::PathBuf;

use derive_variants::EnumVariants;
use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::{
  deployment::DockerContainerState,
  server::stats::{SeverityLevel, SystemProcess},
  update::ResourceTarget,
};

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
#[doc_index(doc! { "data.type": 1 })]
#[doc_index(doc! { "target.type": 1 })]
#[doc_index(doc! { "target.id": 1 })]
pub struct Alert {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,

  #[index]
  pub ts: I64,

  #[index]
  pub resolved: bool,

  #[index]
  pub level: SeverityLevel,

  pub target: ResourceTarget,
  pub variant: AlertDataVariant,
  pub data: AlertData,
  pub resolved_ts: Option<I64>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash
)]
#[serde(tag = "type", content = "data")]
pub enum AlertData {
  ServerUnreachable {
    id: String,
    name: String,
    region: Option<String>,
  },
  ServerCpu {
    id: String,
    name: String,
    region: Option<String>,
    percentage: f64,
    top_procs: Vec<SystemProcess>,
  },
  ServerMem {
    id: String,
    name: String,
    region: Option<String>,
    used_gb: f64,
    total_gb: f64,
    top_procs: Vec<SystemProcess>,
  },
  ServerDisk {
    id: String,
    name: String,
    region: Option<String>,
    path: PathBuf,
    used_gb: f64,
    total_gb: f64,
  },
  ServerTemp {
    id: String,
    name: String,
    region: Option<String>,
    component: String,
    temp: f64,
    max: f64,
  },
  ContainerStateChange {
    id: String,
    name: String,
    server_id: String,
    server_name: String,
    from: DockerContainerState,
    to: DockerContainerState,
  },
  None {},
}

impl Default for AlertData {
  fn default() -> Self {
    AlertData::None {}
  }
}

#[allow(clippy::derivable_impls)]
impl Default for AlertDataVariant {
  fn default() -> Self {
    AlertDataVariant::None
  }
}