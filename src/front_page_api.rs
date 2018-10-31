use ARTSTATION_URL;

use http::Method;
use request::ApiRequestBuilder;
use reqwest::Client;

static TOP_ROW_ITEMS: &str = "/top_row_items.json";
static CAMPAIGN: &str = "/c.json";
static PROJECTS: &str = "/projects.json";
static JOBS: &str = "/jobs.json";

pub struct FrontPageApi<'a> {
    client: &'a Client,
}

impl<'a> FrontPageApi<'a> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> Self {
        FrontPageApi { client }
    }

    pub fn top_row_items(&self) -> ApiRequestBuilder<TopRowItemsRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &[ARTSTATION_URL, TOP_ROW_ITEMS].concat(),
        )
    }
    // has takeover query w/e that does
    pub fn campaign_info(&self) -> ApiRequestBuilder<CampaignInfoRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &[ARTSTATION_URL, CAMPAIGN].concat(),
        )
    }

    pub fn projects(&self) -> ApiRequestBuilder<::users_api::ProjectsRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &[ARTSTATION_URL, PROJECTS].concat(),
        )
    }

    pub fn jobs(&self) -> ApiRequestBuilder<JobsRequest> {
        ApiRequestBuilder::new(
            self.client.clone(),
            Method::GET,
            &[ARTSTATION_URL, JOBS].concat(),
        )
    }
}

use json_def::*;
use request::*;
make_request! {
    TopRowItemsRequest = Vec<TopRowItem> with LimitQuery;
    CampaignInfoRequest = Campaign with SizeQuery, TakeOverQuery;
    JobsRequest = Vec<Job> with FeaturedQuery, LimitQuery;
}
