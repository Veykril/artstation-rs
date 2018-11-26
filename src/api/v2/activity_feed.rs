use crate::{
    api::{
        v2::{request_types::UserFeedsRequest, V2},
        ArtStationApi,
    },
    ApiRequestBuilder, ArtStation,
};

/// This struct reflects the activity feed endpoint. You get an instance by calling the [`activity_feed`]
/// method of the [`V2`] struct.
///
/// [`cart`]: ./struct.V2.html#method.activity_feed
pub struct ActivityFeed<'a> {
    art_client: &'a ArtStation,
}

impl<'a> ActivityFeed<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        ActivityFeed { art_client }
    }

    pub fn user_feeds(&self) -> ApiRequestBuilder<UserFeedsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("user_feeds"))
    }
}

impl<'a> ArtStationApi for ActivityFeed<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [
            ArtStation::URL,
            V2::API_BASE,
            "activity_feed/",
            endpoint,
            ".json",
        ]
        .concat()
    }
}
