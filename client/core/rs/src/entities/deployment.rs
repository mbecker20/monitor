use anyhow::Context;
use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use partial_derive2::Partial;
use serde::{
  de::{value::SeqAccessDeserializer, Visitor},
  Deserialize, Deserializer, Serialize,
};
use strum::{Display, EnumString};
use typeshare::typeshare;

use crate::parser::parse_key_value_list;

use super::{
  docker::container::ContainerStateStatusEnum,
  resource::{Resource, ResourceListItem, ResourceQuery},
  TerminationSignal, Version,
};

#[typeshare]
pub type Deployment = Resource<DeploymentConfig, ()>;

#[typeshare]
pub type DeploymentListItem =
  ResourceListItem<DeploymentListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentListItemInfo {
  /// The state of the deployment / underlying docker container.
  pub state: DeploymentState,
  /// The status of the docker container (eg. up 12 hours, exited 5 minutes ago.)
  pub status: Option<String>,
  /// The image attached to the deployment.
  pub image: String,
  /// The server that deployment sits on.
  pub server_id: String,
  /// An attached Komodo Build, if it exists.
  pub build_id: Option<String>,
}

#[typeshare(serialized_as = "Partial<DeploymentConfig>")]
pub type _PartialDeploymentConfig = PartialDeploymentConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct DeploymentConfig {
  /// The id of server the deployment is deployed on.
  #[serde(default, alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  #[builder(default)]
  pub server_id: String,

  /// The image which the deployment deploys.
  /// Can either be a user inputted image, or a Komodo Build.
  #[serde(default)]
  #[builder(default)]
  pub image: DeploymentImage,

  /// Configure the account used to pull the image from the registry.
  /// Used with `docker login`.
  ///
  ///  - If the field is empty string, will use the same account config as the build, or none at all if using image.
  ///  - If the field contains an account, a token for the account must be available.
  ///  - Will get the registry domain from the build / image
  #[serde(default)]
  #[builder(default)]
  pub image_registry_account: String,

  /// Whether to skip secret interpolation into the deployment environment variables.
  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

  /// Whether to redeploy the deployment whenever the attached build finishes.
  #[serde(default)]
  #[builder(default)]
  pub redeploy_on_build: bool,

  /// Whether to send ContainerStateChange alerts for this deployment.
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_alerts: bool,

  /// Configure quick links that are displayed in the resource header
  #[serde(default)]
  #[builder(default)]
  pub links: Vec<String>,

  /// The network attached to the container.
  /// Default is `host`.
  #[serde(default = "default_network")]
  #[builder(default = "default_network()")]
  #[partial_default(default_network())]
  pub network: String,

  /// The restart mode given to the container.
  #[serde(default)]
  #[builder(default)]
  pub restart: RestartMode,

  /// This is interpolated at the end of the `docker run` command,
  /// which means they are either passed to the containers inner process,
  /// or replaces the container command, depending on use of ENTRYPOINT or CMD in dockerfile.
  /// Empty is no command.
  #[serde(default)]
  #[builder(default)]
  pub command: String,

  /// The default termination signal to use to stop the deployment. Defaults to SigTerm (default docker signal).
  #[serde(default)]
  #[builder(default)]
  pub termination_signal: TerminationSignal,

  /// The termination timeout.
  #[serde(default = "default_termination_timeout")]
  #[builder(default = "default_termination_timeout()")]
  #[partial_default(default_termination_timeout())]
  pub termination_timeout: i32,

  /// Extra args which are interpolated into the `docker run` command,
  /// and affect the container configuration.
  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,

  /// Labels attached to various termination signal options.
  /// Used to specify different shutdown functionality depending on the termination signal.
  #[serde(default, deserialize_with = "term_labels_deserializer")]
  #[partial_attr(serde(
    default,
    deserialize_with = "option_term_labels_deserializer"
  ))]
  #[builder(default)]
  pub term_signal_labels: String,

  /// The container port mapping.
  /// Irrelevant if container network is `host`.
  /// Maps ports on host to ports on container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  #[partial_attr(serde(
    default,
    deserialize_with = "option_conversions_deserializer"
  ))]
  #[builder(default)]
  pub ports: String,

  /// The container volume mapping.
  /// Maps files / folders on host to files / folders in container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  #[partial_attr(serde(
    default,
    deserialize_with = "option_conversions_deserializer"
  ))]
  #[builder(default)]
  pub volumes: String,

  /// The environment variables passed to the container.
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub environment: String,

  /// The docker labels given to the container.
  #[serde(
    default,
    deserialize_with = "super::labels_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_labels_deserializer"
  ))]
  #[builder(default)]
  pub labels: String,
}

impl DeploymentConfig {
  pub fn builder() -> DeploymentConfigBuilder {
    DeploymentConfigBuilder::default()
  }
}

fn default_send_alerts() -> bool {
  true
}

fn default_termination_timeout() -> i32 {
  10
}

fn default_network() -> String {
  String::from("host")
}

impl Default for DeploymentConfig {
  fn default() -> Self {
    Self {
      server_id: Default::default(),
      send_alerts: default_send_alerts(),
      links: Default::default(),
      image: Default::default(),
      image_registry_account: Default::default(),
      skip_secret_interp: Default::default(),
      redeploy_on_build: Default::default(),
      term_signal_labels: Default::default(),
      termination_signal: Default::default(),
      termination_timeout: default_termination_timeout(),
      ports: Default::default(),
      volumes: Default::default(),
      environment: Default::default(),
      labels: Default::default(),
      network: default_network(),
      restart: Default::default(),
      command: Default::default(),
      extra_args: Default::default(),
    }
  }
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, EnumVariants,
)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Display,
  EnumString
)]
#[serde(tag = "type", content = "params")]
pub enum DeploymentImage {
  /// Deploy any external image.
  Image {
    /// The docker image, can be from any registry that works with docker and that the host server can reach.
    #[serde(default)]
    image: String,
  },

  /// Deploy a Komodo Build.
  Build {
    /// The id of the Build
    #[serde(default, alias = "build")]
    build_id: String,
    /// Use a custom / older version of the image produced by the build.
    /// if version is 0.0.0, this means `latest` image.
    #[serde(default)]
    version: Version,
  },
}

impl Default for DeploymentImage {
  fn default() -> Self {
    Self::Image {
      image: Default::default(),
    }
  }
}

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct Conversion {
  /// reference on the server.
  pub local: String,
  /// reference in the container.
  pub container: String,
}

pub fn conversions_from_str(
  input: &str,
) -> anyhow::Result<Vec<Conversion>> {
  parse_key_value_list(input).map(|conversions| {
    conversions
      .into_iter()
      .map(|(local, container)| Conversion { local, container })
      .collect()
  })
}

pub fn conversions_deserializer<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(ConversionVisitor)
}

pub fn option_conversions_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<String>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionConversionVisitor)
}

struct ConversionVisitor;

impl<'de> Visitor<'de> for ConversionVisitor {
  type Value = String;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "string or Vec<Conversion>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let out = v.to_string();
    if out.is_empty() || out.ends_with('\n') {
      Ok(out)
    } else {
      Ok(out + "\n")
    }
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    let res = Vec::<Conversion>::deserialize(
      SeqAccessDeserializer::new(seq),
    )?;
    let res = res
      .iter()
      .map(|Conversion { local, container }| {
        format!("  {local}: {container}")
      })
      .collect::<Vec<_>>()
      .join("\n");
    let extra = if res.is_empty() { "" } else { "\n" };
    Ok(res + extra)
  }
}

struct OptionConversionVisitor;

impl<'de> Visitor<'de> for OptionConversionVisitor {
  type Value = Option<String>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<Conversion>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    ConversionVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    ConversionVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }

  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

/// Variants de/serialized from/to snake_case.
///
/// Eg.
/// - NotDeployed -> not_deployed
/// - Restarting -> restarting
/// - Running -> running.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
  Display,
  EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DeploymentState {
  #[default]
  Unknown,
  NotDeployed,
  Created,
  Restarting,
  Running,
  Removing,
  Paused,
  Exited,
  Dead,
}

impl From<ContainerStateStatusEnum> for DeploymentState {
  fn from(value: ContainerStateStatusEnum) -> Self {
    match value {
      ContainerStateStatusEnum::Empty => DeploymentState::Unknown,
      ContainerStateStatusEnum::Created => DeploymentState::Created,
      ContainerStateStatusEnum::Running => DeploymentState::Running,
      ContainerStateStatusEnum::Paused => DeploymentState::Paused,
      ContainerStateStatusEnum::Restarting => {
        DeploymentState::Restarting
      }
      ContainerStateStatusEnum::Removing => DeploymentState::Removing,
      ContainerStateStatusEnum::Exited => DeploymentState::Exited,
      ContainerStateStatusEnum::Dead => DeploymentState::Dead,
    }
  }
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
  Display,
  EnumString,
)]
pub enum RestartMode {
  #[default]
  #[serde(rename = "no")]
  #[strum(serialize = "no")]
  NoRestart,
  #[serde(rename = "on-failure")]
  #[strum(serialize = "on-failure")]
  OnFailure,
  #[serde(rename = "always")]
  #[strum(serialize = "always")]
  Always,
  #[serde(rename = "unless-stopped")]
  #[strum(serialize = "unless-stopped")]
  UnlessStopped,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Default,
  PartialEq,
  Eq,
  Builder,
)]
pub struct TerminationSignalLabel {
  #[builder(default)]
  pub signal: TerminationSignal,
  #[builder(default)]
  pub label: String,
}

pub fn term_signal_labels_from_str(
  input: &str,
) -> anyhow::Result<Vec<TerminationSignalLabel>> {
  parse_key_value_list(input).and_then(|list| {
    list
      .into_iter()
      .map(|(signal, label)| {
        anyhow::Ok(TerminationSignalLabel {
          signal: signal.parse()?,
          label,
        })
      })
      .collect()
  })
}

pub fn term_labels_deserializer<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(TermSignalLabelVisitor)
}

pub fn option_term_labels_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<String>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionTermSignalLabelVisitor)
}

struct TermSignalLabelVisitor;

impl<'de> Visitor<'de> for TermSignalLabelVisitor {
  type Value = String;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "string or Vec<TerminationSignalLabel>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    let out = v.to_string();
    if out.is_empty() || out.ends_with('\n') {
      Ok(out)
    } else {
      Ok(out + "\n")
    }
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    let res = Vec::<TerminationSignalLabel>::deserialize(
      SeqAccessDeserializer::new(seq),
    )?
    .into_iter()
    .map(|TerminationSignalLabel { signal, label }| {
      format!("  {signal}: {label}")
    })
    .collect::<Vec<_>>()
    .join("\n");
    let extra = if res.is_empty() { "" } else { "\n" };
    Ok(res + extra)
  }
}

struct OptionTermSignalLabelVisitor;

impl<'de> Visitor<'de> for OptionTermSignalLabelVisitor {
  type Value = Option<String>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<TerminationSignalLabel>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    TermSignalLabelVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    TermSignalLabelVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }

  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DeploymentActionState {
  pub deploying: bool,
  pub starting: bool,
  pub restarting: bool,
  pub pausing: bool,
  pub unpausing: bool,
  pub stopping: bool,
  pub destroying: bool,
  pub renaming: bool,
}

#[typeshare]
pub type DeploymentQuery = ResourceQuery<DeploymentQuerySpecifics>;

#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, DefaultBuilder,
)]
pub struct DeploymentQuerySpecifics {
  #[serde(default)]
  pub server_ids: Vec<String>,

  #[serde(default)]
  pub build_ids: Vec<String>,
}

impl super::resource::AddFilters for DeploymentQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.server_ids.is_empty() {
      filters
        .insert("config.server_id", doc! { "$in": &self.server_ids });
    }
    if !self.build_ids.is_empty() {
      filters.insert("config.image.type", "Build");
      filters.insert(
        "config.image.params.build_id",
        doc! { "$in": &self.build_ids },
      );
    }
  }
}

pub fn extract_registry_domain(
  image_name: &str,
) -> anyhow::Result<String> {
  let mut split = image_name.split('/');
  let maybe_domain =
    split.next().context("image name cannot be empty string")?;
  if maybe_domain.contains('.') {
    Ok(maybe_domain.to_string())
  } else {
    Ok(String::from("docker.io"))
  }
}
