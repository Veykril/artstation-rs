// todo figure out unknown fields and optional fields(nullable fields)

use super::nullable_priority;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnknownField;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub followed: bool,
    pub blocked: bool,
    pub is_staff: bool,
    pub badges: Vec<UnknownField>,
    pub collections_count: i64,
    pub has_pro_permissions: bool,
    pub portfolio_display_settings_albums: Vec<UnknownField>,
    pub display_portfolio_as_albums: bool,
    pub show_all_projects_album: Option<UnknownField>,
    pub albums_with_community_projects: Vec<AlbumsWithCommunityProject>,
    pub profile_default_album: ProfileDefaultAlbum,
    pub social_profiles: Vec<SocialProfile>,
    pub skills: Vec<Skill>,
    pub software_items: Vec<SoftwareItem>,
    pub experience_items: Vec<ExperienceItem>,
    pub user_productions: Vec<UnknownField>,
    pub portfolio: Portfolio,
    pub portfolio_display_settings: PortfolioDisplaySettings,
    pub id: i64,
    pub large_avatar_url: String,
    pub medium_avatar_url: String,
    pub default_cover_url: String,
    pub full_name: String,
    pub headline: String,
    pub username: String,
    pub artstation_url: String,
    pub artstation_website: String,
    pub followers_count: i64,
    pub followees_count: i64,
    pub liked_projects_count: i64,
    pub projects_count: i64,
    pub community_projects_count: i64,
    pub project_views_count: i64,
    pub projects_likes_count: i64,
    pub city: String,
    pub country: String,
    pub permalink: String,
    pub cover_file_name: Option<String>,
    #[serde(deserialize_with = "nullable_priority")]
    pub cover_width: i64,
    #[serde(deserialize_with = "nullable_priority")]
    pub cover_height: i64,
    pub availability: String,
    pub available_full_time: bool,
    pub available_contract: bool,
    pub available_freelance: bool,
    pub pro_member: bool,
    pub profile_artstation_website: String,
    pub profile_artstation_website_url: String,
    pub first_name: String,
    pub last_name: String,
    pub memorialized: Option<String>,
    pub twitter_url: Option<String>,
    pub facebook_url: Option<String>,
    pub tumblr_url: Option<String>,
    pub deviantart_url: Option<String>,
    pub google_plus_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub instagram_url: Option<String>,
    pub pinterest_url: Option<String>,
    pub youtube_url: Option<String>,
    pub vimeo_url: Option<String>,
    pub behance_url: Option<String>,
    pub steam_url: Option<String>,
    pub sketchfab_url: Option<String>,
    pub twitch_url: Option<String>,
    pub imdb_url: Option<String>,
    pub website_url: Option<String>,
    pub public_email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumsWithCommunityProject {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub position: i64,
    pub community_projects_count: i64,
    pub total_projects: i64,
    pub website_projects_count: i64,
    pub public_projects_count: i64,
    #[serde(deserialize_with = "nullable_priority")]
    pub profile_visibility: bool,
    #[serde(deserialize_with = "nullable_priority")]
    pub website_visibility: bool,
    pub album_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileDefaultAlbum {
    pub id: i64,
    pub album_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocialProfile {
    pub id: i64,
    pub url: String,
    pub social_network: String,
    pub position: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoftwareItem {
    pub icon_url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExperienceItem {
    pub company: Company,
    pub title: String,
    pub start_date: String,
    pub finish_date: String,
    pub location: String,
    pub description: String,
    pub position: i64,
    pub start_date_formatted: String,
    pub finish_date_formatted: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Company {
    pub icon_url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub summary_as_html: String,
    pub resume_url: Option<String>,
    pub summary: Option<String>,
    pub demo_reel_code: Option<UnknownField>,
    pub demo_reel_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioDisplaySettings {
    pub profile_default_album_id: i64,
    pub website_default_album_id: i64,
    pub display_portfolio_as_albums: bool,
}
