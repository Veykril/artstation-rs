extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate http;
extern crate serde;

pub mod json_def;
#[macro_use]
mod request;
pub use request::{ApiRequestBuilder, Size};
mod front_page_api;
use front_page_api::FrontPageApi;
mod users_api;
use users_api::UserApi;

use reqwest::{Client, Result};

static ARTSTATION_URL: &str = "https://www.artstation.com";

pub struct ArtStation {
    client: Client,
}

impl ArtStation {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(ArtStation {
            client: Client::builder().build()?,
        })
    }

    #[inline]
    pub fn from_client(client: Client) -> Self {
        ArtStation { client }
    }

    #[inline]
    pub fn user<'a, 'b>(&'a self, name: &'b str) -> UserApi<'a, 'b> {
        UserApi::new(&self.client, name)
    }

    #[inline]
    pub fn front_page(&self) -> FrontPageApi {
        FrontPageApi::new(&self.client)
    }
}
