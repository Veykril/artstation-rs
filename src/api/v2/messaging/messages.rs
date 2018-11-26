use crate::api::{
    v2::{
        messaging::{request_types::PermissionsRequest, Messaging},
        V2,
    },
    ArtStationApi,
};
use crate::ApiRequestBuilder;
use crate::ArtStation;

/// This struct reflects the conversations endpoint. You get an instance by calling the [`messages`]
/// method of the [`Messaging`] struct.
///
/// [`conversations`]: ./struct.Messaging.html#method.messages
pub struct Messages<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Messages<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Messages { art_client }
    }

    pub fn permissions(&self) -> ApiRequestBuilder<PermissionsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("permissions"))
    }
}

impl<'a> ArtStationApi for Messages<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            Messaging::API_MESSAGING,
            "messages/",
            endpoint,
            ".json",
        ]
        .concat()
    }
}
