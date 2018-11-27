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
        Ok(response.json::<Self>()?.data())
    }
}

impl ArtStationResponse for () {
    type Output = ();
    fn from_reqwest_response(_response: reqwest::Response) -> Result<Self::Output> {
        Ok(())
    }
}

pub struct RawResponse;

impl ArtStationResponse for RawResponse {
    type Output = reqwest::Response;
    fn from_reqwest_response(response: reqwest::Response) -> Result<Self::Output> {
        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound = "R: DeserializeOwned")]
#[serde = "untagged"]
pub enum JsonPagedResponse<R: DeserializeOwned> {
    Standard {
        total_count: usize,
        data: Vec<R>,
    },
    Detailed {
        per_page: usize,
        page: usize,
        total: usize,
        data: Vec<R>,
    },
}

impl<R: DeserializeOwned> JsonPagedResponse<R> {
    pub fn data(self) -> Vec<R> {
        match self {
            JsonPagedResponse::Standard { data, .. } | JsonPagedResponse::Detailed { data, .. } => {
                data
            }
        }
    }

    pub fn data_mut(&mut self) -> &mut Vec<R> {
        match self {
            JsonPagedResponse::Standard { data, .. } | JsonPagedResponse::Detailed { data, .. } => {
                data
            }
        }
    }

    pub fn total(&self) -> usize {
        match self {
            JsonPagedResponse::Standard {
                total_count: total, ..
            }
            | JsonPagedResponse::Detailed { total, .. } => *total,
        }
    }
}
