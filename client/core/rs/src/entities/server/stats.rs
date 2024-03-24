use std::{collections::HashMap, path::PathBuf};

use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typeshare::typeshare;

use crate::{entities::Timelength, entities::I64};

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
#[collection_name(Stats)]
pub struct SystemStatsRecord {
  #[index]
  pub ts: I64,
  #[index]
  pub sid: String,
  // basic stats
  pub load_average: LoadAverage,
  pub cpu_perc: f32,
  pub cpu_freq_mhz: f64,
  pub mem_used_gb: f64,
  pub mem_total_gb: f64,
  pub disk_used_gb: f64,
  pub disk_total_gb: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemInformation {
  pub name: Option<String>,
  pub os: Option<String>,
  pub kernel: Option<String>,
  pub core_count: Option<u32>,
  pub host_name: Option<String>,
  pub cpu_brand: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AllSystemStats {
  pub basic: BasicSystemStats,
  pub cpu: CpuUsage,
  pub disk: DiskUsage,
  pub network: NetworkUsage,
  #[serde(default)]
  pub processes: Vec<SystemProcess>,
  #[serde(default)]
  pub components: Vec<SystemComponent>,
  pub polling_rate: Timelength,
  pub refresh_ts: I64,
  pub refresh_list_ts: I64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BasicSystemStats {
  pub load_average: LoadAverage,
  pub cpu_perc: f32,
  pub cpu_freq_mhz: f64,
  pub mem_used_gb: f64,
  pub mem_total_gb: f64,
  pub disk_used_gb: f64,
  pub disk_total_gb: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct LoadAverage {
  pub one: f64,
  pub five: f64,
  pub fifteen: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CpuUsage {
  pub cpu_perc: f32,
  pub cpu_freq_mhz: f64,
  #[serde(default)]
  pub cpus: Vec<SingleCpuUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleCpuUsage {
  pub name: String,
  pub usage: f32,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiskUsage {
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
  pub read_kb: f64,  // in kB
  pub write_kb: f64, // in kB
  #[serde(default)]
  pub disks: Vec<SingleDiskUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleDiskUsage {
  pub mount: PathBuf,
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NetworkUsage {
  pub recieved_kb: f64,
  pub transmitted_kb: f64,
  #[serde(default)]
  pub networks: Vec<SystemNetwork>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemNetwork {
  pub name: String,
  pub recieved_kb: f64,    // in kB
  pub transmitted_kb: f64, // in kB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemProcess {
  pub pid: u32,
  pub name: String,
  #[serde(default)]
  pub exe: String,
  pub cmd: Vec<String>,
  #[serde(default)]
  pub start_time: f64,
  pub cpu_perc: f32,
  pub mem_mb: f64,
  pub disk_read_kb: f64,
  pub disk_write_kb: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemComponent {
  pub label: String,
  pub temp: f32,
  pub max: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub critical: Option<f32>,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Default,
  Display,
  EnumString,
  PartialEq,
  Eq,
)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum SeverityLevel {
  #[default]
  Ok,
  Warning,
  Critical,
}

#[typeshare]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ServerHealth {
  pub cpu: SeverityLevel,
  pub mem: SeverityLevel,
  pub disk: SeverityLevel,
  pub disks: HashMap<PathBuf, SeverityLevel>,
  pub temps: HashMap<String, SeverityLevel>,
}