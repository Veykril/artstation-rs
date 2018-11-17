use reqwest::{IntoUrl, Method, RequestBuilder, Result};
use serde::de::DeserializeOwned;

use std::marker::PhantomData;

use ArtStation;

pub trait LimitQuery: ArtStationRequest {}
pub trait PageQuery: ArtStationRequest {}
pub trait TakeOverQuery: ArtStationRequest {}
pub trait SizeQuery: ArtStationRequest {}
pub trait FeaturedQuery: ArtStationRequest {}
pub trait AlbumIdQuery: ArtStationRequest {}

pub trait ArtStationRequest {
    type Response: ArtStationResponse;
}

pub trait ArtStationResponse {
    type Output: Sized;
    fn from_reqwest_response(response: reqwest::Response) -> Result<Self::Output>;
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

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Small,
    Large,
}

#[derive(Debug, Deserialize)]
#[serde(bound = "R: DeserializeOwned")]
pub struct JsonPagedResponse<R: DeserializeOwned> {
    pub total_count: usize,
    pub data: Vec<R>,
}

pub struct ApiRequestBuilder<'a, R> {
    request_builder: RequestBuilder,
    art_client: &'a ArtStation,
    _pd: PhantomData<R>,
}

impl<'a, R: ArtStationRequest> ApiRequestBuilder<'a, R> {
    pub(crate) fn new<U: IntoUrl>(art_client: &'a ArtStation, method: Method, url: U) -> Self {
        ApiRequestBuilder {
            request_builder: art_client.client.request(method, url),
            art_client,
            _pd: PhantomData,
        }
    }

    pub(crate) fn get<U: IntoUrl>(art_client: &'a ArtStation, url: U) -> Self {
        ApiRequestBuilder {
            request_builder: art_client.client.get(url),
            art_client,
            _pd: PhantomData,
        }
    }

    pub(crate) fn post<U: IntoUrl>(art_client: &'a ArtStation, url: U) -> Self {
        ApiRequestBuilder {
            request_builder: art_client.client.post(url),
            art_client,
            _pd: PhantomData,
        }
    }

    pub fn send_raw(self) -> Result<reqwest::Response> {
        self.art_client.send_request(self.request_builder)
    }

    pub fn send(self) -> Result<<R::Response as ArtStationResponse>::Output> {
        R::Response::from_reqwest_response(self.art_client.send_request(self.request_builder)?)
    }
}

impl<'a, T, R> ApiRequestBuilder<'a, R>
where
    T: ArtStationResponse + DeserializeOwned,
    R: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
    pub fn send_all(self) -> Result<Vec<T>> {
        let request = self.request_builder.build()?;
        let mut page = 1;
        let mut response_buf = Vec::new();
        loop {
            let mut response = ApiRequestBuilder::<R>::new(
                self.art_client,
                request.method().clone(),
                request.url().clone(),
            )
            .page(page)
            .send_raw()?
            .json::<R::Response>()?;

            if response_buf.capacity() == 0 {
                response_buf.reserve_exact(response.total_count);
            }
            response_buf.append(&mut response.data);
            if response.total_count == response_buf.len() {
                break Ok(response_buf);
            }
            page += 1;
        }
    }
}

impl<'a, R: LimitQuery> ApiRequestBuilder<'a, R> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("limit", limit)]);
        self
    }
}

impl<'a, R: PageQuery> ApiRequestBuilder<'a, R> {
    pub fn page(mut self, page: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("page", page)]);
        self
    }
}

impl<'a, R: TakeOverQuery> ApiRequestBuilder<'a, R> {
    pub fn takeover(mut self, takeover: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("takeover", takeover)]);
        self
    }
}

impl<'a, R: SizeQuery> ApiRequestBuilder<'a, R> {
    pub fn size(mut self, size: Size) -> Self {
        self.request_builder = self.request_builder.query(&[("size", size)]);
        self
    }
}

impl<'a, R: FeaturedQuery> ApiRequestBuilder<'a, R> {
    pub fn featured(mut self, featured: bool) -> Self {
        self.request_builder = self.request_builder.query(&[("featured", featured)]);
        self
    }
}

impl<'a, R: AlbumIdQuery> ApiRequestBuilder<'a, R> {
    pub fn album_id(mut self, id: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("album_id", id)]);
        self
    }
}

impl<S, T> PageQuery for S
where
    T: ArtStationResponse + DeserializeOwned,
    S: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
}

pub mod request_types {
    use super::*;
    use json_def::*;
    make_request! {
        ProjectsRequest = JsonPagedResponse<Project> with AlbumIdQuery;
        ProfileRequest = Profile;
        FollowersRequest = JsonPagedResponse<Follower>;
        FollowingsRequest = JsonPagedResponse<Follower>;
        SubmissionsRequest = JsonPagedResponse<Submission>;
        LikesRequest = JsonPagedResponse<Like>;
        TopRowItemsRequest = Vec<TopRowItem> with LimitQuery;
        CampaignInfoRequest = Campaign with SizeQuery, TakeOverQuery;
        JobsRequest = Vec<Job> with FeaturedQuery, LimitQuery;
    }
}
