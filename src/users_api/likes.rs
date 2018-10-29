#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    id: i64,
    user_id: i64,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
    likes_count: i64,
    slug: String,
    published_at: String,
    adult_content: bool,
    cover_asset_id: i64,
    admin_adult_content: bool,
    hash_id: String,
    permalink: String,
    hide_as_adult: bool,
    user: User,
    cover: Cover,
    icons: Icons,
    assets_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cover {
    id: i64,
    small_square_url: String,
    micro_square_image_url: String,
    thumb_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Icons {
    image: bool,
    video: bool,
    model3d: bool,
    marmoset: bool,
    pano: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i64,
    username: String,
    first_name: String,
    last_name: String,
    avatar_file_name: String,
    country: String,
    city: String,
    subdomain: String,
    headline: String,
    pro_member: bool,
    is_staff: bool,
    medium_avatar_url: String,
    large_avatar_url: String,
    full_name: String,
    permalink: String,
    artstation_profile_url: String,
    location: String,
}
