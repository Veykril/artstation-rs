use crate::{
    api::{
        v2::{
            request_types::{CountRequest, CurrentRequest},
            V2,
        },
        ArtStationApi,
    },
    ApiRequestBuilder, ArtStation,
};

/// This struct reflects the shopping cart endpoint. You get an instance by calling the [`cart`]
/// method of the [`V2`] struct.
///
/// [`cart`]: ./struct.V2.html#method.cart
pub struct Cart<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Cart<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Cart { art_client }
    }

    pub fn count(&self) -> ApiRequestBuilder<CountRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("count"))
    }

    pub fn current(&self) -> ApiRequestBuilder<CurrentRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("current"))
    }
}

impl<'a> ArtStationApi for Cart<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [ArtStation::URL, V2::API_BASE, "cart/", endpoint, ".json"].concat()
    }
}
