pub mod query;
pub mod response;

use reqwest::{IntoUrl, Method, RequestBuilder, Result};
use serde::de::DeserializeOwned;

use std::marker::PhantomData;

use crate::{
    request::response::{ArtStationResponse, JsonPagedResponse},
    ArtStation,
};

/// An `ArtStationRequest` is used by the [`ApiRequestBuilder`] for it's generic type.
pub trait ArtStationRequest {
    type Response: ArtStationResponse;
}

impl crate::request::ArtStationRequest for () {
    type Response = ();
}

/// An `ApiRequestBuilder` is what you use to make a request. You cannot create one by itself,
/// you have to use the builder functions of the [`ArtStation`] instead to make one. It is generic over
/// its [`ArtStationRequest`] so that you may only append queries that make sense for the given
/// request. To find out what queries you may use, check what [`query`] the request type implements.
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

    #[inline]
    pub(crate) fn get<U: IntoUrl>(art_client: &'a ArtStation, url: U) -> Self {
        Self::new(art_client, Method::GET, url)
    }

    #[inline]
    pub(crate) fn post<U: IntoUrl>(art_client: &'a ArtStation, url: U) -> Self {
        Self::new(art_client, Method::POST, url)
    }

    #[inline]
    pub(crate) fn put<U: IntoUrl>(art_client: &'a ArtStation, url: U) -> Self {
        Self::new(art_client, Method::PUT, url)
    }

    /// Sends this request without converting it to its designated output type, giving you the
    /// unmodified [`reqwest::Response`].
    pub fn send_raw(self) -> Result<reqwest::Response> {
        self.art_client.send_request(self.request_builder)
    }

    /// Sends this request.
    pub fn send(self) -> Result<<R::Response as ArtStationResponse>::Output> {
        R::Response::from_reqwest_response(self.send_raw()?.error_for_status()?)
    }
}

impl<'a, T, R> ApiRequestBuilder<'a, R>
where
    T: ArtStationResponse + DeserializeOwned,
    R: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
    /// This function is special to requests that dont necessarily return the complete data
    /// at once. This will call the `paged`([`PageQuery`]) request until all of the data has
    /// been collected.
    ///
    /// Use this if you dont want to just get one part of a [`PageQuery`]able request.
    ///
    /// [`PageQuery`]: query/trait.PageQuery.html
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
                response_buf.reserve_exact(response.total());
            }
            //let total = response.total();
            response_buf.append(response.data_mut());
            if response.total() == response_buf.len() {
                break Ok(response_buf);
            }
            page += 1;
        }
    }
}
