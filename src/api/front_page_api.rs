use crate::request::{request_types::*, ApiRequestBuilder};
use crate::ArtStation;

static TOP_ROW_ITEMS: &str = "/top_row_items";
static CAMPAIGN: &str = "/c";
static PROJECTS: &str = "/projects";
static JOBS: &str = "/jobs";

pub struct FrontPageApi<'a> {
    art_client: &'a ArtStation,
}

impl<'a> FrontPageApi<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        FrontPageApi { art_client }
    }

    #[inline]
    fn craft_url(&self, endpoint: &str) -> String {
        [ArtStation::URL, endpoint, ".json"].concat()
    }

    pub fn top_row_items(&self) -> ApiRequestBuilder<TopRowItemsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(TOP_ROW_ITEMS))
    }

    pub fn campaign_info(&self) -> ApiRequestBuilder<CampaignInfoRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(CAMPAIGN))
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(PROJECTS))
    }

    pub fn jobs(&self) -> ApiRequestBuilder<JobsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url(JOBS))
    }
}
