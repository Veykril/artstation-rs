use serde::de::DeserializeOwned;

use super::{
    response::{ArtStationResponse, JsonPagedResponse},
    ArtStationRequest,
};

macro_rules! define_query {
    ($($name:ident $raw_name:ident: $ty:ty);*;) => {
        $(
            pub trait $name: super::ArtStationRequest {}

            impl<'a, R: $name> super::ApiRequestBuilder<'a, R> {
                pub fn $raw_name(mut self, $raw_name: $ty) -> Self {
                    self.request_builder = self.request_builder.query(&[(stringify!($raw_name), $raw_name)]);
                    self
                }
            }
        )*
    };
}

define_query! {
    LimitQuery limit: u32;
    PageQuery page: u32;
    TakeOverQuery take_over: u32;
    SizeQuery size: Size;
    FeaturedQuery featured: bool;
    AlbumIdQuery album_id: u32;
    IncludeMarketplaceQuery include_marketplace: bool;
    LatestQuery latest: bool;
    UserId user_id: u32;
    RandomizeQuery randomize: bool;
    CategoryQuery category: Category;
    MediumQuery medium: Medium;
    MarmorSetQuery marmorset: bool;
    PanoQuery pano: bool;
    SketchfabQuery sketchfab: bool;
    VideoQuery video: bool;
    SortingQuery sorting: Sorting;
}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Small,
    Large,
}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Sorting {
    Trending,
    Latest,
    Picks,
    Following,
}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Medium {
    Digital2d,
    Digital3d,
    Traditional2d,
    Traditional3d,
}

#[derive(Serialize)]
#[serde = "untagged"]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Animation,
    #[serde(rename = "archviz")]
    ArchitecturalVisualization,
    Architecture,
    Characters,
    #[serde(rename = "comic_art")]
    ComicArt,
    #[serde(rename = "concept_art")]
    ConceptArt,
    Creatures,
    Environments,
    Fantasy,
    #[serde(rename = "game_art")]
    GameArt,
    Illustration,
    #[serde(rename = "industrial_design")]
    IndustrialDesign,
    #[serde(rename = "matte_painting")]
    MattePainting,
    Mecha,
    MotionGraphics,
    ProductDesign,
    #[serde(rename = "props&page")]
    PropsPage,
    #[serde(rename = "science_fiction")]
    ScienceFiction,
    #[serde(rename = "still_life")]
    StillLife,
    Storyboards,
    Storytelling,
    Surreal,
    TexturesMaterials,
    Transport,
    Tutorial,
    #[serde(rename = "ui")]
    UserInterface,
    #[serde(rename = "vfx")]
    VisualEffects,
    #[serde(rename = "vr")]
    VirtualReality,
    Weapons,
}

impl<S, T> PageQuery for S
where
    T: ArtStationResponse + DeserializeOwned,
    S: ArtStationRequest<Response = JsonPagedResponse<T>>,
{
}
