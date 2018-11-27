use crate::json_def::v2::shared::LastActor;

#[derive(Clone, Debug, Deserialize)]
pub struct UserFeed {
    id: String,
    #[serde(rename = "type")]
    datum_type: Type,
    last_actors: Vec<LastActor>,
    followee: Option<Followee>,
    project: Option<Project>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Followee {
    id: i64,
    username: String,
    location: String,
    default_cover_url: String,
    large_avatar_url: String,
    pro_member: bool,
    followed_by_me: bool,
    followers_count: i64,
    project_likes_count: i64,
    artstation_profile_url: String,
    is_staff: bool,
    full_name: String,
    headline: String,
    sample_projects: Vec<SampleProject>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SampleProject {
    id: i64,
    cover_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    id: i64,
    url: String,
    adult_content: bool,
    admin_adult_content: bool,
    suppressed: bool,
    editor_pick: bool,
    small_cover_url: String,
    large_cover_url: String,
    asset_count: i64,
    published_at: String,
    views_count: i64,
    likes_count: i64,
    liked: bool,
    hash_id: String,
    assets: Vec<Asset>,
    title: String,
    icons: Icons,
    description: String,
    user: LastActor,
    comments: Comments,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    id: i64,
    asset_type: AssetType,
    width: i64,
    height: i64,
    image_url: String,
    small_image_url: String,
    large_image_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Comments {
    total_count: i64,
    data: Vec<ChildCommentElement>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChildCommentElement {
    id: i64,
    text_as_html: String,
    likes_count: i64,
    liked: bool,
    hidden_by_user: bool,
    text: String,
    created_at: String,
    user: User,
    child_comments: Option<Vec<ChildCommentElement>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    id: i64,
    username: String,
    full_name: String,
    headline: String,
    medium_avatar_url: String,
    pro_member: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Icons {
    video: bool,
    image: bool,
    #[serde(rename = "model3d")]
    model3_d: bool,
    marmoset: bool,
    pano: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Type {
    #[serde(rename = "followee_followed")]
    FolloweeFollowed,
    #[serde(rename = "project_liked")]
    ProjectLiked,
}

#[derive(Clone, Debug, Deserialize)]
pub enum AssetType {
    #[serde(rename = "image")]
    Image,
}
