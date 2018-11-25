use crate::api::{
    v2::{messaging::Messaging, request_types::UnreadCountRequest, V2},
    ArtStationApi,
};
use crate::ApiRequestBuilder;
use crate::ArtStation;

/// This struct reflects the conversations endpoint. You get an instance by calling the [`conversations`]
/// method of the [`Messaging`] struct.
///
/// [`conversations`]: ./struct.Messaging.html#method.conversations
pub struct Conversations<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Conversations<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Conversations { art_client }
    }

    pub fn unread_count(&self) -> ApiRequestBuilder<UnreadCountRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("unread_count"))
    }
}

impl<'a> ArtStationApi for Conversations<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            Messaging::API_MESSAGING,
            "conversations/",
            endpoint,
            ".json",
        ]
        .concat()
    }
}
