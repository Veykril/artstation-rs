#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
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
    pub user: User,
    pub cover: Cover,
    pub icons: Icons,
    pub assets_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cover {
    pub id: i64,
    pub small_square_url: String,
    pub micro_square_image_url: String,
    pub thumb_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Icons {
    pub image: bool,
    pub video: bool,
    pub model3d: bool,
    pub marmoset: bool,
    pub pano: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_file_name: String,
    pub country: String,
    pub city: String,
    pub subdomain: String,
    pub headline: String,
    pub pro_member: bool,
    pub is_staff: bool,
    pub medium_avatar_url: String,
    pub large_avatar_url: String,
    pub full_name: String,
    pub permalink: String,
    pub artstation_profile_url: String,
    pub location: String,
}
