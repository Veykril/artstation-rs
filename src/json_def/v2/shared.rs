#[derive(Debug, Clone, Deserialize)]
pub struct LastActor {
    id: i64,
    username: String,
    full_name: String,
    pro_member: bool,
    is_staff: bool,
    memorialized: Option<bool>,
    medium_avatar_url: String,
    small_cover_url: Option<String>,
    headline: String,
    artstation_profile_url: Option<String>,
}
