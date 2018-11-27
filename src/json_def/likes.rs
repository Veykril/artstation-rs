use super::shared::{Cover, Icons};
use crate::json_def::projects::Project;
use crate::json_def::shared::User;

#[derive(Clone, Debug, Deserialize)]
pub struct Like {
    pub user: User,
    #[serde(flatten)]
    pub project: Project,
}
