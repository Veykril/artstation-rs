#[derive(Debug, Deserialize, Serialize)]
pub struct Follower {
    pub id: i64,
    pub username: String,
    pub avatar_file_name: Option<String>,
    pub country: String,
    pub city: String,
    pub likes_count: i64,
    pub subdomain: String,
    pub headline: String,
    pub available_full_time: bool,
    pub available_contract: bool,
    pub available_freelance: bool,
    pub followers_count: i64,
    pub pro_member: bool,
    pub artist_role: bool,
    pub followed: bool,
    pub full_name: String,
    pub medium_avatar_url: String,
    pub large_avatar_url: String,
    pub is_staff: bool,
    pub sample_projects: Vec<String>,
    pub skills: Vec<Skill>,
    pub software_items: Vec<SoftwareItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoftwareItem {
    pub name: String,
    pub icon_url: String,
}
