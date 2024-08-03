use anyhow::anyhow;
use command::run_monitor_command;
use monitor_client::entities::{
  deployment::{ContainerSummary, DockerContainerStats},
  to_monitor_name,
  update::Log,
  SearchCombinator,
};
use periphery_client::api::container::*;
use resolver_api::Resolve;

use crate::{
  docker::{container_stats, docker_client, stop_container_command},
  State,
};

// ======
//  READ
// ======

//

impl Resolve<GetContainerList> for State {
  #[instrument(
    name = "GetContainerList",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    _: GetContainerList,
    _: (),
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    docker_client().list_containers().await
  }
}

//

impl Resolve<GetContainerLog> for State {
  #[instrument(name = "GetContainerLog", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetContainerLog { name, tail }: GetContainerLog,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = format!("docker logs {name} --tail {tail}");
    Ok(run_monitor_command("get container log", command).await)
  }
}

//

impl Resolve<GetContainerLogSearch> for State {
  #[instrument(
    name = "GetContainerLogSearch",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    GetContainerLogSearch {
      name,
      terms,
      combinator,
      invert,
    }: GetContainerLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    let maybe_invert = invert.then_some(" -v").unwrap_or_default();
    let grep = match combinator {
      SearchCombinator::Or => {
        format!("grep{maybe_invert} -E '{}'", terms.join("|"))
      }
      SearchCombinator::And => {
        format!(
          "grep{maybe_invert} -P '^(?=.*{})'",
          terms.join(")(?=.*")
        )
      }
    };
    let command =
      format!("docker logs {name} --tail 5000 2>&1 | {grep}");
    Ok(run_monitor_command("get container log grep", command).await)
  }
}

//

impl Resolve<GetContainerStats> for State {
  #[instrument(
    name = "GetContainerStats",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    req: GetContainerStats,
    _: (),
  ) -> anyhow::Result<DockerContainerStats> {
    let error = anyhow!("no stats matching {}", req.name);
    let mut stats = container_stats(Some(req.name)).await?;
    let stats = stats.pop().ok_or(error)?;
    Ok(stats)
  }
}

//

impl Resolve<GetContainerStatsList> for State {
  #[instrument(
    name = "GetContainerStatsList",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    _: GetContainerStatsList,
    _: (),
  ) -> anyhow::Result<Vec<DockerContainerStats>> {
    container_stats(None).await
  }
}

// =========
//  ACTIONS
// =========

impl Resolve<StartContainer> for State {
  #[instrument(name = "StartContainer", skip(self))]
  async fn resolve(
    &self,
    StartContainer { name }: StartContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      run_monitor_command(
        "docker start",
        format!("docker start {name}"),
      )
      .await,
    )
  }
}

//

impl Resolve<RestartContainer> for State {
  #[instrument(name = "RestartContainer", skip(self))]
  async fn resolve(
    &self,
    RestartContainer { name }: RestartContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      run_monitor_command(
        "docker restart",
        format!("docker restart {name}"),
      )
      .await,
    )
  }
}

//

impl Resolve<PauseContainer> for State {
  #[instrument(name = "PauseContainer", skip(self))]
  async fn resolve(
    &self,
    PauseContainer { name }: PauseContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      run_monitor_command(
        "docker pause",
        format!("docker pause {name}"),
      )
      .await,
    )
  }
}

impl Resolve<UnpauseContainer> for State {
  #[instrument(name = "UnpauseContainer", skip(self))]
  async fn resolve(
    &self,
    UnpauseContainer { name }: UnpauseContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      run_monitor_command(
        "docker unpause",
        format!("docker unpause {name}"),
      )
      .await,
    )
  }
}

//

impl Resolve<StopContainer> for State {
  #[instrument(name = "StopContainer", skip(self))]
  async fn resolve(
    &self,
    StopContainer { name, signal, time }: StopContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = stop_container_command(&name, signal, time);
    let log = run_monitor_command("docker stop", command).await;
    if log.stderr.contains("unknown flag: --signal") {
      let command = stop_container_command(&name, None, time);
      let mut log = run_monitor_command("docker stop", command).await;
      log.stderr = format!(
        "old docker version: unable to use --signal flag{}",
        if !log.stderr.is_empty() {
          format!("\n\n{}", log.stderr)
        } else {
          String::new()
        }
      );
      Ok(log)
    } else {
      Ok(log)
    }
  }
}

//

impl Resolve<RemoveContainer> for State {
  #[instrument(name = "RemoveContainer", skip(self))]
  async fn resolve(
    &self,
    RemoveContainer { name, signal, time }: RemoveContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let stop_command = stop_container_command(&name, signal, time);
    let command =
      format!("{stop_command} && docker container rm {name}");
    let log =
      run_monitor_command("docker stop and remove", command).await;
    if log.stderr.contains("unknown flag: --signal") {
      let stop_command = stop_container_command(&name, None, time);
      let command =
        format!("{stop_command} && docker container rm {name}");
      let mut log = run_monitor_command("docker stop", command).await;
      log.stderr = format!(
        "old docker version: unable to use --signal flag{}",
        if !log.stderr.is_empty() {
          format!("\n\n{}", log.stderr)
        } else {
          String::new()
        }
      );
      Ok(log)
    } else {
      Ok(log)
    }
  }
}

//

impl Resolve<RenameContainer> for State {
  #[instrument(name = "RenameContainer", skip(self))]
  async fn resolve(
    &self,
    RenameContainer {
      curr_name,
      new_name,
    }: RenameContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let new = to_monitor_name(&new_name);
    let command = format!("docker rename {curr_name} {new}");
    Ok(run_monitor_command("docker rename", command).await)
  }
}

//

impl Resolve<PruneContainers> for State {
  #[instrument(name = "PruneContainers", skip(self))]
  async fn resolve(
    &self,
    _: PruneContainers,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = String::from("docker container prune -f");
    Ok(run_monitor_command("prune containers", command).await)
  }
}
