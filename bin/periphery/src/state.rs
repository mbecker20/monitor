use std::{
  collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc,
};

use anyhow::Context;
use clap::Parser;
use serde_json::json;

use crate::{
  config::{CliArgs, Env, PeripheryConfig},
  helpers::{
    docker::DockerClient,
    stats::{InnerStatsClient, StatsClient},
  },
};

pub struct State {
  pub config: PeripheryConfig,
  pub secrets: Arc<HashMap<String, String>>,
  pub stats: StatsClient,
  pub docker: DockerClient,
  pub accounts_response: String,
  pub secrets_response: String,
}

impl State {
  pub async fn load() -> anyhow::Result<Arc<State>> {
    let env = Env::load()?;
    let args = CliArgs::parse();
    logger::init(args.log_level)?;
    let config = PeripheryConfig::load(&env, &args)?;
    let state = State {
      secrets: config.secrets.clone().into(),
      docker: DockerClient::default(),
      stats: InnerStatsClient::new(config.stats_polling_rate),
      accounts_response: serde_json::to_string(&json!({
          "docker": config.docker_accounts.keys().collect::<Vec<_>>(),
          "github": config.github_accounts.keys().collect::<Vec<_>>(),
      }))
      .unwrap(),
      secrets_response: serde_json::to_string(
        &config.secrets.keys().collect::<Vec<_>>(),
      )
      .unwrap(),
      config,
    };
    Ok(state.into())
  }

  pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
    SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
      .context("failed to parse socket addr")
  }
}