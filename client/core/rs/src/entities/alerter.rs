use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{
  alert::AlertDataVariant,
  resource::{Resource, ResourceListItem, ResourceQuery},
  ResourceTarget,
};

#[typeshare]
pub type Alerter = Resource<AlerterConfig, ()>;

#[typeshare]
pub type AlerterListItem = ResourceListItem<AlerterListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlerterListItemInfo {
  /// Whether alerter is enabled for sending alerts
  pub enabled: bool,
  /// The type of the alerter, eg. `Slack`, `Custom`
  pub endpoint_type: AlerterEndpointVariant,
}

#[typeshare(serialized_as = "Partial<AlerterConfig>")]
pub type _PartialAlerterConfig = PartialAlerterConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct AlerterConfig {
  /// Whether the alerter is enabled
  #[serde(default = "default_enabled")]
  #[builder(default = "default_enabled()")]
  #[partial_default(default_enabled())]
  pub enabled: bool,

  /// Where to route the alert messages.
  ///
  /// Default: Custom endpoint `http://localhost:7000`
  #[serde(default)]
  #[builder(default)]
  pub endpoint: AlerterEndpoint,

  /// Only send specific alert types.
  /// If empty, will send all alert types.
  #[serde(default)]
  #[builder(default)]
  pub alert_types: Vec<AlertDataVariant>,

  /// Only send alerts on specific resources.
  /// If empty, will send alerts for all resources.
  #[serde(default)]
  #[builder(default)]
  pub resources: Vec<ResourceTarget>,

  /// DON'T send alerts on these resources.
  #[serde(default)]
  #[builder(default)]
  pub except_resources: Vec<ResourceTarget>,
}

impl AlerterConfig {
  pub fn builder() -> AlerterConfigBuilder {
    AlerterConfigBuilder::default()
  }
}

fn default_enabled() -> bool {
  true
}

impl Default for AlerterConfig {
  fn default() -> Self {
    Self {
      enabled: default_enabled(),
      endpoint: Default::default(),
      alert_types: Default::default(),
      resources: Default::default(),
      except_resources: Default::default(),
    }
  }
}

// ENDPOINTS

#[typeshare]
#[derive(
  Debug, Clone, PartialEq, Serialize, Deserialize, EnumVariants,
)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "params")]
pub enum AlerterEndpoint {
  /// Send alert serialized to JSON to an http endpoint.
  Custom(CustomAlerterEndpoint),

  /// Send alert to a slack app
  Slack(SlackAlerterEndpoint),
}

impl Default for AlerterEndpoint {
  fn default() -> Self {
    Self::Custom(Default::default())
  }
}

/// Configuration for a custom alerter endpoint.
#[typeshare]
#[derive(
  Debug, Clone, PartialEq, Serialize, Deserialize, Builder,
)]
pub struct CustomAlerterEndpoint {
  /// The http/s endpoint to send the POST to
  #[serde(default = "default_custom_url")]
  #[builder(default = "default_custom_url()")]
  pub url: String,
}

impl Default for CustomAlerterEndpoint {
  fn default() -> Self {
    Self {
      url: default_custom_url(),
    }
  }
}

fn default_custom_url() -> String {
  String::from("http://localhost:7000")
}

/// Configuration for a slack alerter.
#[typeshare]
#[derive(
  Debug, Clone, PartialEq, Serialize, Deserialize, Builder,
)]
pub struct SlackAlerterEndpoint {
  /// The slack app url
  #[serde(default = "default_slack_url")]
  #[builder(default = "default_slack_url()")]
  pub url: String,
}

impl Default for SlackAlerterEndpoint {
  fn default() -> Self {
    Self {
      url: default_slack_url(),
    }
  }
}

fn default_slack_url() -> String {
  String::from(
    "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX",
  )
}

// QUERY

#[typeshare]
pub type AlerterQuery = ResourceQuery<AlerterQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct AlerterQuerySpecifics {
  /// Filter alerters by enabled.
  /// - `None`: Don't filter by enabled
  /// - `Some(true)`: Only include alerts with `enabled: true`
  /// - `Some(false)`: Only include alerts with `enabled: false`
  pub enabled: Option<bool>,

  /// Only include alerters with these endpoint types.
  /// If empty, don't filter by enpoint type.
  pub types: Vec<AlerterEndpointVariant>,
}

impl super::resource::AddFilters for AlerterQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if let Some(enabled) = self.enabled {
      filters.insert("config.enabled", enabled);
    }
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.endpoint.type", doc! { "$in": types });
    }
  }
}
