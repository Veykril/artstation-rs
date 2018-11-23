use crate::api::v2::{messaging::Messaging, UnreadCountRequest, V2};
use crate::ApiRequestBuilder;
use crate::ArtStation;

pub struct Conversations<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Conversations<'a> {
    const API_CONVERSATIONS: &'static str = "conversations/";

    #[inline]
    pub(crate) fn make_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            Messaging::API_MESSAGING,
            Self::API_CONVERSATIONS,
            endpoint,
            ".json",
        ]
        .concat()
    }

    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Conversations { art_client }
    }

    pub fn unread_count(&self) -> ApiRequestBuilder<UnreadCountRequest> {
        ApiRequestBuilder::get(self.art_client, &self.make_url("unread_count"))
    }
}
