#[derive(Debug, Deserialize, Serialize)]
pub struct Campaign {
    campaign_asset: CampaignAsset,
    id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CampaignAsset {
    width: i64,
    height: i64,
    image_url: String,
    impression_tracking_tag: Option<String>,
    asset_type: String,
    asset_size: String,
    id: i64,
    url: String,
    embed_html: Option<String>,
}
