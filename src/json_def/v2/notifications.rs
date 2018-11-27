use crate::json_def::v2::shared::LastActor;

#[derive(Clone, Debug, Deserialize)]
pub struct Notification {
    id: i64,
    last_actor_image_url: String,
    last_actors: Vec<LastActor>,
    entity: Entity,
    message: String,
    created_at_ago: String,
    group: Group,
    preview_html: String,
    quote_text: Option<()>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Entity {
    project_id: Option<i64>,
    challenge_id: Option<i64>,
    submission_id: Option<i64>,
    submission_update_id: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Group {
    #[serde(rename = "artwork")]
    Artwork,
    #[serde(rename = "challenges")]
    Challenges,
}
