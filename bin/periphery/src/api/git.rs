use anyhow::{anyhow, Context};
use git::GitRes;
use komodo_client::entities::{
  to_komodo_name, update::Log, CloneArgs, LatestCommit,
};
use periphery_client::api::git::{
  CloneRepo, DeleteRepo, GetLatestCommit, PullOrCloneRepo, PullRepo,
  RepoActionResponse,
};
use resolver_api::Resolve;

use crate::{config::periphery_config, State};

impl Resolve<GetLatestCommit, ()> for State {
  async fn resolve(
    &self,
    GetLatestCommit { name }: GetLatestCommit,
    _: (),
  ) -> anyhow::Result<LatestCommit> {
    let repo_path = periphery_config().repo_dir.join(name);
    if !repo_path.is_dir() {
      return Err(anyhow!(
        "Repo path is not directory. is it cloned?"
      ));
    }
    git::get_commit_hash_info(&repo_path).await
  }
}

impl Resolve<CloneRepo> for State {
  #[instrument(
    name = "CloneRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    CloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: CloneRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::clone(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<PullRepo> for State {
  #[instrument(
    name = "PullRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    PullRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::pull(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<PullOrCloneRepo> for State {
  #[instrument(
    name = "PullOrCloneRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    PullOrCloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullOrCloneRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::pull_or_clone(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<DeleteRepo> for State {
  #[instrument(name = "DeleteRepo", skip(self))]
  async fn resolve(
    &self,
    DeleteRepo { name }: DeleteRepo,
    _: (),
  ) -> anyhow::Result<Log> {
    let name = to_komodo_name(&name);
    let deleted = std::fs::remove_dir_all(
      periphery_config().repo_dir.join(&name),
    );
    let msg = match deleted {
      Ok(_) => format!("deleted repo {name}"),
      Err(_) => format!("no repo at {name} to delete"),
    };
    Ok(Log::simple("delete repo", msg))
  }
}
