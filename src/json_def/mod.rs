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
pub mod v2;

use reqwest::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

use crate::request::response::ArtStationResponse;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataWrap<T: Sized> {
    data: T,
}

impl<T> ArtStationResponse for DataWrap<T>
where
    T: ArtStationResponse + DeserializeOwned,
{
    type Output = T;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json::<Self>()?.data)
    }
}

impl_generic_json_response! {
    Campaign, Follower, TopRowItem, Job, Like, Profile, Project, Submission
}
