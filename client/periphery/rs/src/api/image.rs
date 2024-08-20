use monitor_client::entities::{
  docker::image::{Image, ImageListItem},
  update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<ImageListItem>)]
pub struct GetImageList {}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Image)]
pub struct InspectImage {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneImages {}