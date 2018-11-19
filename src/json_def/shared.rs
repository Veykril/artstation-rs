#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Icons {
    pub image: bool,
    pub video: bool,
    pub model3d: bool,
    pub marmoset: bool,
    pub pano: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cover {
    pub id: i64,
    pub small_square_url: String,
    pub micro_square_image_url: String,
    pub thumb_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SoftwareItem {
    pub name: String,
    pub icon_url: String,
}
