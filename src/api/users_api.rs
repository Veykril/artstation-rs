use super::request_types::*;
use crate::{api::ArtStationApi, request::ApiRequestBuilder, ArtStation};

/// This struct offers builder methods for anything related to user profiles. You get an instance by
/// calling the [`user`] method of the ArtStation struct. The client does not have to be logged in
/// for any of these requests.
///
/// [`user`]: ../struct.ArtStation.html#method.user
pub struct UsersApi<'a, 'b> {
    art_client: &'a ArtStation,
    name: &'b str,
}

impl<'a, 'b> UsersApi<'a, 'b> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation, name: &'b str) -> Self {
        UsersApi { art_client, name }
    }

    pub fn profile(&self) -> ApiRequestBuilder<ProfileRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(""))
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/projects"))
    }

    pub fn likes(&self) -> ApiRequestBuilder<LikesRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/likes"))
    }

    pub fn followers(&self) -> ApiRequestBuilder<FollowersRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/followers"))
    }

    pub fn followings(&self) -> ApiRequestBuilder<FollowingsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/followings"))
    }

    pub fn submissions(&self) -> ApiRequestBuilder<SubmissionsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/submissions"))
    }

    pub fn random_projects(&self) -> ApiRequestBuilder<RandomProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/random_projects"))
    }

    pub fn hover_card(&self) -> ApiRequestBuilder<HoverCardRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/hovercard"))
    }

    pub fn quick(&self) -> ApiRequestBuilder<QuickRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/quick"))
    }

    /* This request ignores the name and requires you to be logged in
    pub fn summary(&self) -> ApiRequestBuilder<SummaryRequest> {
        ApiRequestBuilder::get(self.art_client, &[ArtStation::URL, "/users/summary.json"].concat())
    }*/
}

impl<'a, 'b> ArtStationApi for UsersApi<'a, 'b> {
    fn craft_url(&self, endpoint: &str) -> String {
        [ArtStation::URL, "/users/", self.name, endpoint, ".json"].concat()
    }
}
