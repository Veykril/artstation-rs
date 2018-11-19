use v2::{V2, UnreadCountRequest};
use ApiRequestBuilder;
use ArtStation;

pub struct Notifications<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Notifications<'a> {
    const API_NOTIFICATIONS: &'static str = "notifications/";

    #[inline]
    pub(crate) fn make_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            Self::API_NOTIFICATIONS,
            endpoint,
            ".json",
        ]
        .concat()
    }

    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Notifications { art_client }
    }

    pub fn unread_count(&self) -> ApiRequestBuilder<UnreadCountRequest> {
        ApiRequestBuilder::get(self.art_client, &self.make_url("unread_count"))
    }
}