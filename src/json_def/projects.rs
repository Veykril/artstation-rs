use super::shared::{Cover, Icons};

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub likes_count: i64,
    pub slug: String,
    pub published_at: String,
    pub adult_content: bool,
    pub cover_asset_id: i64,
    pub admin_adult_content: bool,
    pub hash_id: String,
    pub permalink: String,
    pub hide_as_adult: bool,
    pub cover: Cover,
    pub icons: Icons,
    pub assets_count: i64,
}
