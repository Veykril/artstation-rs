use crate::api::ArtStationApi;
use crate::request::{request_types::*, ApiRequestBuilder};
use crate::ArtStation;

static PROJECTS: &str = "/projects.json";
static FOLLOWERS: &str = "/followers.json";
static FOLLOWINGS: &str = "/followings.json";
static LIKES: &str = "/likes.json";
static SUBMISSIONS: &str = "/likes.json";
static PROFILE: &str = ".json";

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
        ApiRequestBuilder::get(self.art_client, &self.craft_url(PROFILE))
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(PROJECTS))
    }

    pub fn likes(&self) -> ApiRequestBuilder<LikesRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(LIKES))
    }

    pub fn followers(&self) -> ApiRequestBuilder<FollowersRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(FOLLOWERS))
    }

    pub fn followings(&self) -> ApiRequestBuilder<FollowingsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(FOLLOWINGS))
    }

    pub fn submissions(&self) -> ApiRequestBuilder<SubmissionsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(SUBMISSIONS))
    }
}

impl<'a, 'b> ArtStationApi for UsersApi<'a, 'b> {
    fn craft_url(&self, endpoint: &str) -> String {
        [ArtStation::URL, "/users/", self.name, endpoint, ".json"].concat()
    }
}
