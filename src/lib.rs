extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod json_def;

mod front_page_api;
mod request;
mod users_api;
mod v2;

use regex::Regex;
use reqwest::{header::COOKIE, Client, RedirectPolicy, Result};

use std::cell::RefCell;

pub use front_page_api::FrontPageApi;
pub use request::{request_types, ApiRequestBuilder, query};
pub use users_api::UserApi;
pub use v2::V2;

static SIGN_IN: &str = "/users/sign_in";

pub struct ArtStation {
    client: Client,
    session: RefCell<Option<String>>,
}

impl ArtStation {
    const URL: &'static str = "https://www.artstation.com";
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(ArtStation {
            client: Client::builder().redirect(RedirectPolicy::none()).build()?,
            session: RefCell::new(None),
        })
    }

    pub fn login(&self, email: &str, password: &str) -> Result<reqwest::Response> {
        lazy_static! {
            static ref TOKEN_REGEX: Regex =
                Regex::new(r#"name="authenticity_token" value="(.*?)""#).unwrap();
        }

        let mut login_prep_response =
            self.send_request(self.client.get(&[Self::URL, SIGN_IN].concat()))?;
        let html = login_prep_response.text().unwrap();

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

        self.send_request(
            self.client
                .post(&[Self::URL, SIGN_IN].concat())
                .form(&params),
        )
    }

    #[inline]
    pub fn v2(&self) -> V2 {
        V2::new(self)
    }

    #[inline]
    pub fn user<'a, 'b>(&'a self, name: &'b str) -> UserApi<'a, 'b> {
        UserApi::new(self, name)
    }

    #[inline]
    pub fn front_page(&self) -> FrontPageApi {
        FrontPageApi::new(self)
    }

    pub fn send_request(&self, mut builder: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        if let Some(session) = self.session.borrow().clone() {
            builder = builder.header(COOKIE, session);
        }
        let response = builder.send()?;
        if let Some(new_session) = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .find(|x| x.as_bytes().starts_with(b"_ArtStation_session="))
            .map(|v| v.to_str().ok().map(ToOwned::to_owned))
            .unwrap_or_default()
        {
            self.session.replace(Some(new_session));
        }
        Ok(response)
    }
}
