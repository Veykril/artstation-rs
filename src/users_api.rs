use ARTSTATION_URL;

use http::Method;
use request::{ApiRequestBuilder, request_types::*};
use reqwest::Client;

static USERS_API: &str = "/users/";
static PROJECTS: &str = "/projects.json";
static FOLLOWERS: &str = "/followers.json";
static FOLLOWINGS: &str = "/followings.json";
static LIKES: &str = "/likes.json";
static SUBMISSIONS: &str = "/likes.json";
static PROFILE: &str = ".json";

pub struct UserApi<'a, 'b> {
    client: &'a Client,
    name: &'b str,
}

impl<'a, 'b> UserApi<'a, 'b> {
    pub(crate) fn new(client: &'a Client, name: &'b str) -> Self {
        UserApi { client, name }
    }

    #[inline]
    fn craft_url(&self, endpoint: &str) -> String {
        [ARTSTATION_URL, USERS_API, self.name, endpoint].concat()
    }

    pub fn profile(&self) -> ApiRequestBuilder<ProfileRequest> {
        ApiRequestBuilder::new(self.client.clone(), Method::GET, &self.craft_url(PROFILE))
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::new(self.client.clone(), Method::GET, &self.craft_url(PROJECTS))
    }

    pub fn likes(&self) -> ApiRequestBuilder<LikesRequest> {
        ApiRequestBuilder::new(self.client.clone(), Method::GET, &self.craft_url(LIKES))
    }

    pub fn followers(&self) -> ApiRequestBuilder<FollowersRequest> {
        ApiRequestBuilder::new(self.client.clone(), Method::GET, &self.craft_url(FOLLOWERS))
    }

    pub fn followings(&self) -> ApiRequestBuilder<FollowingsRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &self.craft_url(FOLLOWINGS),
        )
    }

    pub fn submissions(&self) -> ApiRequestBuilder<SubmissionsRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &self.craft_url(SUBMISSIONS),
        )
    }
}