use request::ArtStationResponse;
use reqwest::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnreadCount {
    data: UnreadCountInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnreadCountInner {
    unread: i64,
}

impl ArtStationResponse for UnreadCount {
    type Output = i64;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json::<Self>()?.data.unread)
    }
}
