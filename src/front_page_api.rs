use request::{request_types::*, ApiRequestBuilder};

use ArtStation;
use ARTSTATION_URL;

static TOP_ROW_ITEMS: &str = "/top_row_items.json";
static CAMPAIGN: &str = "/c.json";
static PROJECTS: &str = "/projects.json";
static JOBS: &str = "/jobs.json";

pub struct FrontPageApi<'a> {
    art_client: &'a ArtStation,
}

impl<'a> FrontPageApi<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        FrontPageApi { art_client }
    }

    pub fn top_row_items(&self) -> ApiRequestBuilder<TopRowItemsRequest> {
        ApiRequestBuilder::get(self.art_client, &[ARTSTATION_URL, TOP_ROW_ITEMS].concat())
    }
    // has takeover query w/e that does
    pub fn campaign_info(&self) -> ApiRequestBuilder<CampaignInfoRequest> {
        ApiRequestBuilder::get(self.art_client, &[ARTSTATION_URL, CAMPAIGN].concat())
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &[ARTSTATION_URL, PROJECTS].concat())
    }

    pub fn jobs(&self) -> ApiRequestBuilder<JobsRequest> {
        ApiRequestBuilder::get(self.art_client, &[ARTSTATION_URL, JOBS].concat())
    }
}
