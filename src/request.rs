use reqwest::{Client, IntoUrl, Method, RequestBuilder, Result};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub trait LimitQuery: ArtStationRequest {}
pub trait PageQuery: ArtStationRequest {}
pub trait TakeOverQuery: ArtStationRequest {}
pub trait SizeQuery: ArtStationRequest {}
pub trait FeaturedQuery: ArtStationRequest {}
pub trait AlbumIdQuery: ArtStationRequest {}

pub trait ArtStationRequest {
    type Response: ArtStationResponse;
}

pub trait ArtStationResponse: Sized {
    fn from_reqwest_response(response: reqwest::Response) -> Result<Self>;
}

impl<T> ArtStationResponse for T
where
    T: DeserializeOwned,
{
    fn from_reqwest_response(mut response: reqwest::Response) -> Result<Self> {
        Ok(response.json()?)
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

pub struct ApiRequestBuilder<R> {
    request_builder: RequestBuilder,
    client: Client,
    _pd: PhantomData<R>,
}

impl<R: ArtStationRequest> ApiRequestBuilder<R> {
    pub(crate) fn new<U: IntoUrl>(client: Client, method: Method, url: U) -> Self {
        ApiRequestBuilder {
            request_builder: client.request(method, url),
            client,
            _pd: PhantomData,
        }
    }

    pub fn send(self) -> Result<R::Response> {
        R::Response::from_reqwest_response(self.request_builder.send()?)
    }
}

impl<T, R> ApiRequestBuilder<R>
where
    T: ArtStationResponse + DeserializeOwned,
    R: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
    pub fn send_all(self) -> Result<Vec<T>> {
        let request = self.request_builder.build()?;
        let mut page = 1;
        let mut response_buf = Vec::new();
        loop {
            let mut response = R::Response::from_reqwest_response(
                self.client
                    .request(request.method().clone(), request.url().clone())
                    .query(&[("page", page)])
                    .send()?,
            )?;

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

impl<R: LimitQuery> ApiRequestBuilder<R> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("limit", limit)]);
        self
    }
}

impl<R: PageQuery> ApiRequestBuilder<R> {
    pub fn page(mut self, page: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("page", page)]);
        self
    }
}

impl<R: TakeOverQuery> ApiRequestBuilder<R> {
    pub fn takeover(mut self, takeover: u32) -> Self {
        self.request_builder = self.request_builder.query(&[("takeover", takeover)]);
        self
    }
}

impl<R: SizeQuery> ApiRequestBuilder<R> {
    pub fn size(mut self, size: Size) -> Self {
        self.request_builder = self.request_builder.query(&[("size", size)]);
        self
    }
}

impl<R: FeaturedQuery> ApiRequestBuilder<R> {
    pub fn featured(mut self, featured: bool) -> Self {
        self.request_builder = self.request_builder.query(&[("featured", featured)]);
        self
    }
}

impl<R: AlbumIdQuery> ApiRequestBuilder<R> {
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

macro_rules! make_request {
    () => {};
    ($name:ident = $response:ident; $($tail:tt)*) => {
        pub struct $name;
        impl ::request::ArtStationRequest for $name {
            type Response = $response;
        }
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident<$inner:ident>; $($tail:tt)*) => {
        pub struct $name;
        impl ::request::ArtStationRequest for $name {
            type Response = $response<$inner>;
        }
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident with $($query:ident),*; $($tail:tt)*) => {
        make_request!($name = $response;);
        $(
            impl $query for $name {}
        )*
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident<$inner:ident> with $($query:ident),*; $($tail:tt)*) => {
        make_request!($name = $response<$inner>;);
        $(
            impl $query for $name {}
        )*
        make_request!($($tail)*);
    };
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
