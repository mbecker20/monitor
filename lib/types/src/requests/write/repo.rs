use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::repo::{PartialRepoConfig, Repo};

#[typeshare(serialized_as = "Partial<RepoConfig>")]
type _PartialRepoConfig = PartialRepoConfig;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Repo)]
pub struct CreateRepo {
    pub name: String,
    pub config: _PartialRepoConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Repo)]
pub struct CopyRepo {
    pub name: String,
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Repo)]
pub struct DeleteRepo {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Repo)]
pub struct UpdateRepo {
    pub id: String,
    pub config: _PartialRepoConfig,
}