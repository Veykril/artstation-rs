mod activity_feed;
pub use self::activity_feed::*;
mod messages;
pub use self::messages::*;
mod notifications;
pub use self::notifications::*;
mod shared;
pub use self::shared::*;

use reqwest::Result;

use crate::request::response::ArtStationResponse;

#[derive(Clone, Debug, Deserialize)]
pub struct Current {
    total_amount: i64,
    cart_items: Vec<()>, //unknown
}

#[derive(Clone, Debug, Deserialize)]
pub struct Count {
    count: i64,
}

impl ArtStationResponse for Count {
    type Output = i64;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json::<Self>()?.count)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnreadCount {
    data: UnreadCountInner,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnreadCountInner {
    unread: i64,
}

impl ArtStationResponse for UnreadCount {
    type Output = i64;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json::<Self>()?.data.unread)
    }
}

impl_generic_json_response! {
    Current, UserFeed, Permissions, Notification
}
