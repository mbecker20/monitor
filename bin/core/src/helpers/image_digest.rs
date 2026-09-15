use std::{sync::Arc, time::Duration};

use komodo_client::entities::{
  ImageDigest, SwarmOrServer, komodo_timestamp,
};
use mogh_cache::CloneCache;
use periphery_client::api::docker::{
  GetLatestImageCreated, GetLatestImageDigest,
};
use tracing::warn;

use crate::helpers::swarm_or_server_request;

/// Maps images -> (digest, valid until milliseconds)
pub struct ImageDigestCache(CloneCache<String, (ImageDigest, i64)>);

impl ImageDigestCache {
  /// Also spawns a task to periodically clean up expired image digests.
  pub fn new() -> Arc<ImageDigestCache> {
    let cache = Arc::new(ImageDigestCache(Default::default()));
    let clone = cache.clone();
    tokio::spawn(async move {
      let mut interval =
        tokio::time::interval(Duration::from_secs(60 * 60));
      interval.tick().await;
      loop {
        interval.tick().await;
        let ts = komodo_timestamp();
        clone
          .0
          .retain(|_, (_, valid_until)| *valid_until > ts)
          .await;
      }
    });
    cache
  }

  pub async fn get(
    &self,
    swarm_or_server: &SwarmOrServer,
    image: &String,
    account: Option<String>,
    token: Option<String>,
  ) -> anyhow::Result<ImageDigest> {
    if let Some((digest, valid_until)) = self.0.get(image).await
      // Ensure the query time was within last 10 mins to use cache.
      && valid_until > komodo_timestamp()
    {
      return Ok(digest);
    }

    let digest = swarm_or_server_request(
      swarm_or_server,
      GetLatestImageDigest {
        name: image.clone(),
        account,
        token,
      },
    )
    .await?
    .digest;

    let digest = ImageDigest::new(image, &digest);

    self
      .0
      .insert(
        image,
        (digest.clone(), komodo_timestamp() + 10 * 60 * 1_000),
      )
      .await;

    Ok(digest)
  }
}

/// Checks the latest image age against `min_update_age_hours`.
/// Fails open (`true`) if it's `0`, or the age can't be determined.
pub async fn image_meets_min_age(
  swarm_or_server: &SwarmOrServer,
  image: &str,
  account: Option<String>,
  token: Option<String>,
  min_update_age_hours: u32,
) -> anyhow::Result<bool> {
  if min_update_age_hours == 0 {
    return Ok(true);
  }

  let res = swarm_or_server_request(
    swarm_or_server,
    GetLatestImageCreated {
      name: image.to_string(),
      account,
      token,
    },
  )
  .await?;

  let Some(created) = res.created else {
    return Ok(true);
  };

  let Ok(created) = chrono::DateTime::parse_from_rfc3339(&created)
  else {
    warn!(
      "Failed to parse image creation time '{created}' for {image}"
    );
    return Ok(true);
  };

  let age_ms = komodo_timestamp() - created.timestamp_millis();
  let min_age_ms = min_update_age_hours as i64 * 60 * 60 * 1_000;

  Ok(age_ms >= min_age_ms)
}
