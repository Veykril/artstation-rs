use crate::api::v2::{messaging::Messaging, UnreadCountRequest, V2};
use crate::api::ArtStationApi;
use crate::ApiRequestBuilder;
use crate::ArtStation;

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
