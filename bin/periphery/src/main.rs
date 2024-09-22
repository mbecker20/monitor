#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum_server::tls_openssl::OpenSSLConfig;

mod api;
mod compose;
mod config;
mod docker;
mod helpers;
mod router;
mod stats;

struct State;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = config::periphery_config();
  logger::init(&config.logging)?;

  info!("Komodo Periphery version: v{}", env!("CARGO_PKG_VERSION"));
  info!("{:?}", config.sanitized());

  stats::spawn_system_stats_polling_threads();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", config.port))
      .context("failed to parse socket addr")?;

  let app = router::router()
    .into_make_service_with_connect_info::<SocketAddr>();

  if config.ssl_enabled {
    info!("Komodo Periphery starting on https://{}", socket_addr);
    let ssl_config =
      OpenSSLConfig::from_pem_file(&config.ssl_cert, &config.ssl_key)
        .context("Failed to parse ssl ")?;
    axum_server::bind_openssl(socket_addr, ssl_config)
      .serve(app)
      .await?
  } else {
    info!("Komodo Periphery starting on http://{}", socket_addr);
    axum_server::bind(socket_addr).serve(app).await?
  }

  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;

  let app = tokio::spawn(app());

  tokio::select! {
    res = app => return res?,
    _ = term_signal.recv() => {},
  }

  Ok(())
}
