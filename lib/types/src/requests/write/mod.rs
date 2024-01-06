mod alerter;
mod build;
mod builder;
mod deployment;
mod description;
mod launch;
mod permissions;
mod repo;
mod secret;
mod server;
mod tags;
mod user;

pub use alerter::*;
pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use description::*;
pub use launch::*;
pub use permissions::*;
pub use repo::*;
use resolver_api::HasResponse;
pub use secret::*;
pub use server::*;
pub use tags::*;
pub use user::*;

pub trait MonitorWriteRequest: HasResponse {}