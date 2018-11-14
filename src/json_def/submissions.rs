#[derive(Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub user_id: i64,
    pub challenge_id: i64,
    pub likes_count: i64,
    pub views_count: i64,
    pub activity_count: i64,
    pub cover_image_url: String,
    pub winner_position: String,
    pub visible_on_profile: bool,
    pub user: User,
    pub challenge: Challenge,
}

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub id: i64,
    pub title: String,
    pub contest: Contest,
}

#[derive(Serialize, Deserialize)]
pub struct Contest {
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
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
