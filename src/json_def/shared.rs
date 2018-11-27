#[derive(Clone, Debug, Deserialize)]
pub struct Icons {
    pub image: bool,
    pub video: bool,
    pub model3d: bool,
    pub marmoset: bool,
    pub pano: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cover {
    pub id: i64,
    pub small_square_url: String,
    pub micro_square_image_url: String,
    pub thumb_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Skill {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SoftwareItem {
    pub name: String,
    pub icon_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_file_name: Option<String>,
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
