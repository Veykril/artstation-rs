#[derive(Clone, Debug, Deserialize)]
pub struct Permissions {
    allowed: bool,
    intro_text: String,
    enabled_conversation_types: String,
    last_conversation_id: Option<()>,
}
