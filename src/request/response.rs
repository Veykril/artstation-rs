use reqwest::Result;
use serde::de::DeserializeOwned;

pub trait ArtStationResponse: Sized {
    type Output: Sized;
    fn from_reqwest_response(response: reqwest::Response) -> ::reqwest::Result<Self::Output>;
}

impl<T> ArtStationResponse for Vec<T>
where
    T: ArtStationResponse + DeserializeOwned,
{
    type Output = Self;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json()?)
    }
}

impl<T> ArtStationResponse for JsonPagedResponse<T>
where
    T: ArtStationResponse + DeserializeOwned,
{
    type Output = Vec<T>;
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self::Output> {
        Ok(response.json::<Self>()?.data)
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound = "R: DeserializeOwned")]
pub struct JsonPagedResponse<R: DeserializeOwned> {
    pub total_count: usize,
    pub data: Vec<R>,
}
