#[derive(Debug, Deserialize, Serialize)]
pub struct Follower {
    id: i64,
    username: String,
    avatar_file_name: Option<String>,
    country: String,
    city: String,
    likes_count: i64,
    subdomain: String,
    headline: String,
    available_full_time: bool,
    available_contract: bool,
    available_freelance: bool,
    followers_count: i64,
    pro_member: bool,
    artist_role: bool,
    followed: bool,
    full_name: String,
    medium_avatar_url: String,
    large_avatar_url: String,
    is_staff: bool,
    sample_projects: Vec<String>,
    skills: Vec<Skill>,
    software_items: Vec<SoftwareItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Skill {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoftwareItem {
    name: String,
    icon_url: String,
}
