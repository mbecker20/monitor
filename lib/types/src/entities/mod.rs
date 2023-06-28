use std::collections::HashMap;

use anyhow::{anyhow, Context};
use mungos::MungosIndexed;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::optional_string;

pub mod build;
pub mod builder;
pub mod deployment;
pub mod repo;
pub mod server;
pub mod update;
pub mod user;

#[typeshare]
pub type PermissionsMap = HashMap<String, PermissionLevel>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, MungosIndexed)]
pub struct SystemCommand {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub command: String,
}

impl SystemCommand {
    pub fn command(&self) -> Option<String> {
        if self.path.is_empty() || self.command.is_empty() {
            None
        } else {
            Some(format!("cd {} && {}", self.path, self.command))
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, MungosIndexed)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals = value
            .split('.')
            .map(|v| anyhow::Ok(v.parse().context("failed at parsing value into i32")?))
            .collect::<anyhow::Result<Vec<i32>>>()?;
        let version = Version {
            major: *vals
                .first()
                .ok_or(anyhow!("must include at least major version"))?,
            minor: *vals.get(1).unwrap_or(&0),
            patch: *vals.get(2).unwrap_or(&0),
        };
        Ok(version)
    }
}

impl Version {
    pub fn increment(&mut self) {
        self.patch += 1;
    }

    pub fn is_none(&self) -> bool {
        self.major == 0 && self.minor == 0 && self.patch == 0
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CloneArgs {
    pub name: String,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub on_clone: Option<SystemCommand>,
    pub on_pull: Option<SystemCommand>,
    pub github_account: Option<String>,
}

impl From<&self::build::Build> for CloneArgs {
    fn from(build: &self::build::Build) -> CloneArgs {
        CloneArgs {
            name: build.name.clone(),
            repo: optional_string(&build.config.repo),
            branch: optional_string(&build.config.branch),
            on_clone: build.config.pre_build.clone().into(),
            on_pull: None,
            github_account: optional_string(&build.config.github_account),
        }
    }
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy, Default,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
    #[serde(rename = "1-sec")]
    #[strum(serialize = "1-sec")]
    OneSecond,
    #[serde(rename = "5-sec")]
    #[strum(serialize = "5-sec")]
    FiveSeconds,
    #[serde(rename = "10-sec")]
    #[strum(serialize = "10-sec")]
    TenSeconds,
    #[serde(rename = "15-sec")]
    #[strum(serialize = "15-sec")]
    FifteenSeconds,
    #[serde(rename = "30-sec")]
    #[strum(serialize = "30-sec")]
    ThirtySeconds,
    #[default]
    #[serde(rename = "1-min")]
    #[strum(serialize = "1-min")]
    OneMinute,
    #[serde(rename = "2-min")]
    #[strum(serialize = "2-min")]
    TwoMinutes,
    #[serde(rename = "5-min")]
    #[strum(serialize = "5-min")]
    FiveMinutes,
    #[serde(rename = "10-min")]
    #[strum(serialize = "10-min")]
    TenMinutes,
    #[serde(rename = "15-min")]
    #[strum(serialize = "15-min")]
    FifteenMinutes,
    #[serde(rename = "30-min")]
    #[strum(serialize = "30-min")]
    ThirtyMinutes,
    #[serde(rename = "1-hr")]
    #[strum(serialize = "1-hr")]
    OneHour,
    #[serde(rename = "2-hr")]
    #[strum(serialize = "2-hr")]
    TwoHours,
    #[serde(rename = "6-hr")]
    #[strum(serialize = "6-hr")]
    SixHours,
    #[serde(rename = "8-hr")]
    #[strum(serialize = "8-hr")]
    EightHours,
    #[serde(rename = "12-hr")]
    #[strum(serialize = "12-hr")]
    TwelveHours,
    #[serde(rename = "1-day")]
    #[strum(serialize = "1-day")]
    OneDay,
    #[serde(rename = "3-day")]
    #[strum(serialize = "3-day")]
    ThreeDay,
    #[serde(rename = "1-wk")]
    #[strum(serialize = "1-wk")]
    OneWeek,
    #[serde(rename = "2-wk")]
    #[strum(serialize = "2-wk")]
    TwoWeeks,
    #[serde(rename = "30-day")]
    #[strum(serialize = "30-day")]
    ThirtyDays,
}

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString,
    Hash,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionLevel {
    #[default]
    None,
    Read,
    Execute,
    Update,
}

impl Default for &PermissionLevel {
    fn default() -> Self {
        &PermissionLevel::None
    }
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Hash, Eq, Clone, Copy, MungosIndexed,
)]
pub enum Operation {
    // do nothing
    #[default]
    None,

    // server
    CreateServer,
    UpdateServer,
    DeleteServer,
    RenameServer,
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    CreateBuild,
    UpdateBuild,
    DeleteBuild,
    RunBuild,

    // deployment
    CreateDeployment,
    UpdateDeployment,
    DeleteDeployment,
    DeployContainer,
    StopContainer,
    StartContainer,
    RemoveContainer,
    PullDeployment,
    RecloneDeployment,
    RenameDeployment,

    // procedure
    CreateProcedure,
    UpdateProcedure,
    DeleteProcedure,

    // command
    CreateCommand,
    UpdateCommand,
    DeleteCommand,
    RunCommand,

    // group
    CreateGroup,
    UpdateGroup,
    DeleteGroup,

    // user
    ModifyUserEnabled,
    ModifyUserCreateServerPermissions,
    ModifyUserCreateBuildPermissions,
    ModifyUserPermissions,

    // github webhook automation
    AutoBuild,
    AutoPull,
}