use request::ArtStationRequest;
use request::ArtStationResponse;
use serde::de::DeserializeOwned;
use request::JsonPagedResponse;
use ApiRequestBuilder;

pub trait LimitQuery: ArtStationRequest {}
pub trait PageQuery: ArtStationRequest {}
pub trait TakeOverQuery: ArtStationRequest {}
pub trait SizeQuery: ArtStationRequest {}
pub trait FeaturedQuery: ArtStationRequest {}
pub trait AlbumIdQuery: ArtStationRequest {}
pub trait IncludeMarketPlaceQuery: ArtStationRequest {}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Small,
    Large,
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

impl<'a, R: IncludeMarketPlaceQuery> ApiRequestBuilder<'a, R> {
    pub fn include_marketplace(mut self, include: bool) -> Self {
        self.request_builder = self
            .request_builder
            .query(&[("include_marketplace", include)]);
        self
    }
}

impl<S, T> PageQuery for S
    where
        T: ArtStationResponse + DeserializeOwned,
        S: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
}
