use crate::json_def::shared::User;

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Challenge {
    pub id: i64,
    pub title: String,
    pub contest: Contest,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Contest {
    pub slug: String,
    pub title: String,
}
