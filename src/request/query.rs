use serde::de::DeserializeOwned;

use super::{
    response::{ArtStationResponse, JsonPagedResponse},
    ArtStationRequest,
};

macro_rules! define_query {
    ($($name:ident $raw_name:ident: $ty:ty);*;) => {
        $(
            pub trait $name: super::ArtStationRequest {}

            impl<'a, R: $name> super::ApiRequestBuilder<'a, R> {
                pub fn $raw_name(mut self, $raw_name: $ty) -> Self {
                    self.request_builder = self.request_builder.query(&[(stringify!($raw_name), $raw_name)]);
                    self
                }
            }
        )*
    };
}

define_query! {
    LimitQuery limit: u32;
    PageQuery page: u32;
    TakeOverQuery take_over: u32;
    SizeQuery size: Size;
    FeaturedQuery featured: bool;
    AlbumIdQuery album_id: u32;
    IncludeMarketplaceQuery include_marketplace: bool;
}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Small,
    Large,
}

impl<S, T> PageQuery for S
where
    T: ArtStationResponse + DeserializeOwned,
    S: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
}
