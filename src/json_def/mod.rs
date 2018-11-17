mod projects;
pub use self::projects::*;
mod profile;
pub use self::profile::*;
mod likes;
pub use self::likes::*;
mod followers;
pub use self::followers::*;
mod submissions;
pub use self::submissions::*;
mod campaign;
pub use self::campaign::*;
mod item;
pub use self::item::*;
mod jobs;
pub use self::jobs::*;
mod shared;
pub use self::shared::*;

use serde::{Deserialize, Deserializer};

#[allow(dead_code)]
pub(crate) fn nullable_priority<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnknownField;

impl_generic_json_response! {
    Campaign, Follower, TopRowItem, Job, Like, Profile, Project, Submission
}
