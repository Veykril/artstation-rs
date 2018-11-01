extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate http;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate lazy_static;

pub mod json_def;
#[macro_use]
mod request;
pub use request::{ApiRequestBuilder, Size};
mod front_page_api;
use front_page_api::FrontPageApi;
mod users_api;
use regex::Regex;
use reqwest::{Client, Result};
use users_api::UserApi;

static ARTSTATION_URL: &str = "https://www.artstation.com";
static SIGN_IN: &str = "/users/sign_in";

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

    pub fn login(&mut self, email: &str, password: &str) -> Result<reqwest::Response> {
        lazy_static! {
            static ref TOKEN_REGEX: Regex =
                Regex::new(r#"<meta name="csrf-token" content="(.*?)""#).unwrap();
        }
        let mut response = self.client.get(ARTSTATION_URL).send()?;
        let html = response.text().unwrap();
        let headers = response.headers();
        let mut params = std::collections::HashMap::with_capacity(7);
        params.insert("utf8", "✓");
        params.insert(
            "authenticity_token",
            TOKEN_REGEX
                .captures(&html)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
        params.insert("user_return_to", "/");
        params.insert("user[email]", email);
        params.insert("user[password]", password);
        params.insert("user[remember_me]", "true");
        params.insert("button", "");
        let request = self
            .client
            .post(&[ARTSTATION_URL, SIGN_IN].concat())
            .form(&params);
        request.send()
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
