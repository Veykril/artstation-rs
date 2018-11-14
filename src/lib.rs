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
use reqwest::RedirectPolicy;
use reqwest::{header::COOKIE, Client, Result};
use users_api::UserApi;

static ARTSTATION_URL: &str = "https://www.artstation.com";
static SIGN_IN: &str = "/users/sign_in";

pub struct ArtStation {
    client: Client,
    token: Option<String>,
}

impl ArtStation {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(ArtStation {
            client: Client::builder().redirect(RedirectPolicy::none()).build()?,
            token: None,
        })
    }

    pub fn login(&mut self, email: &str, password: &str) -> Result<reqwest::Response> {
        lazy_static! {
            static ref TOKEN_REGEX: Regex =
                Regex::new(r#"name="authenticity_token" value="(.*?)""#).unwrap();
        }

        let mut login_prep_response = self
            .client
            .get(&[ARTSTATION_URL, SIGN_IN].concat())
            .send()?;
        let html = login_prep_response.text().unwrap();

        let login_prep_headers = login_prep_response.headers();
        let artstation_session = login_prep_headers
            .get_all("set-cookie")
            .iter()
            .filter_map(|x| x.to_str().ok())
            .find(|x| x.starts_with("_ArtStation_session="))
            .expect("`_ArtStation_session=` cookie not found, this is a bug");

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

        let mut login_response = self
            .client
            .post(&[ARTSTATION_URL, SIGN_IN].concat())
            .form(&params)
            .header(COOKIE, artstation_session)
            .send()?;
        Ok(login_response)
    }

    //#[inline]
    //pub fn from_client(client: Client) -> Self {
    //    ArtStation { client, token: None }
    //}

    #[inline]
    pub fn user<'a, 'b>(&'a self, name: &'b str) -> UserApi<'a, 'b> {
        UserApi::new(&self.client, name)
    }

    #[inline]
    pub fn front_page(&self) -> FrontPageApi {
        FrontPageApi::new(&self.client)
    }
}
