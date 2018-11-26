use crate::{
    api::{
        v2::{request_types::UnreadCountRequest, V2},
        ArtStationApi,
    },
    ApiRequestBuilder, ArtStation,
};

/// This struct reflects the notifications endpoint. You get an instance by calling the [`notifications`]
/// method of the [`V2`] struct.
///
/// [`notifications`]: ./struct.V2.html#method.notifications
pub struct Notifications<'a> {
    art_client: &'a ArtStation,
}

impl<'a> Notifications<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        Notifications { art_client }
    }

    pub fn unread_count(&self) -> ApiRequestBuilder<UnreadCountRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("unread_count"))
    }

    pub fn mark_all_as_read(&self) -> ApiRequestBuilder<()> {
        ApiRequestBuilder::put(self.art_client, &self.craft_url("mark_all_as_read"))
    }
}

impl<'a> ArtStationApi for Notifications<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            "notifications/",
            endpoint,
            ".json",
        ]
        .concat()
    }
}
