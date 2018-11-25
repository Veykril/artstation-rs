mod front_page_api;
mod users_api;
pub mod v2;

pub use self::front_page_api::FrontPageApi;
pub use self::users_api::UsersApi;

pub(crate) trait ArtStationApi {
    fn craft_url(&self, endpoint: &str) -> String;
}
