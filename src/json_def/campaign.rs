#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Campaign {
    pub campaign_asset: CampaignAsset,
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CampaignAsset {
    pub width: i64,
    pub height: i64,
    pub image_url: String,
    pub impression_tracking_tag: Option<String>,
    pub asset_type: String,
    pub asset_size: String,
    pub id: i64,
    pub url: String,
    pub embed_html: Option<String>,
}
