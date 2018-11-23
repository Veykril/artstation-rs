pub mod query;
pub mod response;

use reqwest::{IntoUrl, Method, RequestBuilder, Result};
use serde::de::DeserializeOwned;

use std::marker::PhantomData;

use crate::request::response::{ArtStationResponse, JsonPagedResponse};
use crate::ArtStation;

pub trait ArtStationRequest {
    type Response: ArtStationResponse;
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
        R::Response::from_reqwest_response(self.send_raw()?.error_for_status()?)
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

pub mod request_types {
    use super::{query::*, *};
    use crate::json_def::*;
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
