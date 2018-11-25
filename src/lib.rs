//! This crate provides serveral [`ApiRequestBuilder`] for the unofficial ArtStation api.
//!
//! # Authentication
//!
//! To authenticate, you will need to create an instance of the [`ArtStation`] struct via it's [`new`]
//! method and then login with your email and password. Some requests can also be executed without
//! logging in, namely: [`UsersApi`] and [`FrontPageApi`]
//!
//! [`new`]: ./struct.ArtStation.html#method.new
//!
//! ```rust,no_run
//! # extern crate artstation;
//! # use artstation::ArtStation;
//! # fn main() {
//!     let artstation = ArtStation::new().unwrap();
//!     artstation.login("your@mail.com", "hunter2");
//! # }
//! ```
//!
//! # Making a Request
//!
//! Requests are done by creating [`ApiRequestBuilder`] to make the correct request:
//!
//! ```rust,no_run
//! # extern crate artstation;
//! # use artstation::{ArtStation, RequestBuilder};
//! # fn main() {
//! #   let artstation = ArtStation::new().unwrap();
//! #   artstation.login("your@mail.com", "hunter2");
//!     let work = artstation.user("someguy").profile().send().unwrap();
//! # }
//! ```

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod api;
pub mod json_def;

mod request;
pub use crate::request::{query, ApiRequestBuilder, ArtStationRequest};

use regex::Regex;
use reqwest::{header::COOKIE, Client, RedirectPolicy, Result};

use std::cell::RefCell;

use crate::api::{v2::V2, FrontPageApi, UsersApi};

static SIGN_IN: &str = "/users/sign_in";

/// An `ArtStation` instance represents a current (possibly logged in) session to the website.
pub struct ArtStation {
    client: Client,
    session: RefCell<Option<String>>,
}

impl ArtStation {
    const URL: &'static str = "https://www.artstation.com";

    /// Creates a new [`ArtStation`] instance.
    /// This function fails when [`reqwest::ClientBuilder`] fails.
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(ArtStation {
            client: Client::builder().redirect(RedirectPolicy::none()).build()?,
            session: RefCell::new(None),
        })
    }

    /// Creates a new [`ArtStation`] instance from a given [`reqwest::ClientBuilder`].
    /// This function fails when [`reqwest::ClientBuilder`] fails.
    #[inline]
    pub fn with_client_builder(builder: reqwest::ClientBuilder) -> Result<Self> {
        Ok(ArtStation {
            client: builder.redirect(RedirectPolicy::none()).build()?,
            session: RefCell::new(None),
        })
    }

    /// Sends a login request with given email and password.
    pub fn login(&self, email: &str, password: &str) -> Result<reqwest::Response> {
        lazy_static! {
            static ref TOKEN_REGEX: Regex =
                Regex::new(r#"name="authenticity_token" value="(.*?)""#).unwrap();
        }

        self.session.replace(None);
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
        )?
        .error_for_status()
    }

    /// Creates a [`V2`]-Builder. All requests from this require the user to be logged in.
    #[inline]
    pub fn v2(&self) -> V2 {
        V2::new(self)
    }

    /// Creates a [`UsersApi`]-Builder. Requests from this do not require the user to be logged in.
    #[inline]
    pub fn user<'a, 'b>(&'a self, name: &'b str) -> UsersApi<'a, 'b> {
        UsersApi::new(self, name)
    }

    /// Creates a [`UsersApi`]-Builder. Requests from this do not require the user to be logged in.
    #[inline]
    pub fn front_page(&self) -> FrontPageApi {
        FrontPageApi::new(self)
    }

    /// Sends a request from a [`reqwest::RequestBuilder`] and looks for a new session cookie in the response to update the current
    /// one. Usually you do not need to call this yourself unless you wanna make a request this
    /// crate doesn't let you do.
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
