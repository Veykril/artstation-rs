use super::{request_types::*, ArtStationApi};
use crate::{request::ApiRequestBuilder, ArtStation};

/// This struct offers builder methods for anything related to the front page of the site. You get
/// an instance by calling the [`front_page`] method of the ArtStation struct. The client does not
/// have to be logged in for any of these requests.
///
/// [`front_page`]: ../struct.ArtStation.html#method.front_page
pub struct FrontPageApi<'a> {
    art_client: &'a ArtStation,
}

impl<'a> FrontPageApi<'a> {
    #[inline]
    pub(crate) fn new(art_client: &'a ArtStation) -> Self {
        FrontPageApi { art_client }
    }

    pub fn top_row_items(&self) -> ApiRequestBuilder<TopRowItemsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/top_row_items"))
    }

    pub fn campaign_info(&self) -> ApiRequestBuilder<CampaignInfoRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/c"))
    }

    pub fn projects(&self) -> ApiRequestBuilder<ProjectsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/PROJECTS"))
    }

    pub fn jobs(&self) -> ApiRequestBuilder<JobsRequest> {
        ApiRequestBuilder::get(self.art_client, &self.craft_url("/JOBS"))
    }
}

impl<'a> ArtStationApi for FrontPageApi<'a> {
    fn craft_url(&self, endpoint: &str) -> String {
        [ArtStation::URL, endpoint, ".json"].concat()
    }
}
