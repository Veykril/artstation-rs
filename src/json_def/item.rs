#[derive(Clone, Debug, Deserialize)]
pub struct TopRowItem {
    pub image_url: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub is_target_blank: bool,
    pub is_target_top: bool,
}
