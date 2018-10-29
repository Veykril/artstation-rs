extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod error;
pub use error::{Error, Result};
pub mod users_api;

use reqwest::Client;

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
        ArtStation {
            client,
        }
    }

    #[inline]
    pub fn get_user_projects(&self, name: &str) -> Result<Vec<users_api::Project>> {
        users_api::get_user_projects(&self.client, name)
    }

    #[inline]
    pub fn get_user_likes(&self, name: &str) -> Result<Vec<users_api::Like>> {
        users_api::get_user_likes(&self.client, name)
    }

    #[inline]
    pub fn get_user_followers(&self, name: &str) -> Result<Vec<users_api::Follower>> {
        users_api::get_user_followers(&self.client, name)
    }

    #[inline]
    pub fn get_user_followings(&self, name: &str) -> Result<Vec<users_api::Follower>> {
        users_api::get_user_followings(&self.client, name)
    }

    #[inline]
    pub fn get_user_about(&self, name: &str) -> Result<users_api::Profile> {
        users_api::get_user_about(&self.client, name)
    }
}
