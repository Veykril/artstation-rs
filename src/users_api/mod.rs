mod projects;
pub use self::projects::*;
mod profile;
pub use self::profile::*;
mod likes;
pub use self::likes::*;
mod followers;
pub use self::followers::*;

use error::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

static ARTSTATION_USERS: &str = "https://www.artstation.com/users_api/";
static PROJECTS_JSON: &str = "/projects.json";
static FOLLOWERS_JSON: &str = "/followers.json";
static FOLLOWINGS_JSON: &str = "/followings.json";
static LIKES_JSON: &str = "/likes.json";
// special, not paged
static PROFILE_JSON: &str = ".json";

fn craft_users_url(name: &str, json: &str) -> Result<String> {
    if name.is_empty() {
        Err(Error::EmptyString)
    } else {
        Ok([ARTSTATION_USERS, name, json].concat())
    }
}

pub(crate) fn nullable_priority<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Debug, Deserialize)]
#[serde(bound = "T: DeserializeOwned")]
pub(crate) struct JsonPagedResponse<T: DeserializeOwned> {
    pub total_count: usize,
    pub data: Vec<T>,
}

pub(crate) fn get_paged_json_data<T: DeserializeOwned>(
    client: &Client,
    url: &str,
) -> Result<Vec<T>> {
    let mut page = 1;
    let mut projects = Vec::new();
    loop {
        let mut response = client.get(url).query(&[("page", page)]).send()?;
        let mut projects_response = response.json::<JsonPagedResponse<T>>()?;

        if projects.capacity() == 0 {
            projects.reserve_exact(projects_response.total_count);
        }
        projects.append(&mut projects_response.data);
        if projects_response.total_count == projects.len() {
            break Ok(projects);
        }
        page += 1;
    }
}

//todo generic overloading, care Followers and Following share the same struct

pub fn get_user_projects(client: &Client, name: &str) -> Result<Vec<Project>> {
    get_paged_json_data(client, &craft_users_url(name, PROJECTS_JSON)?)
}

pub fn get_user_likes(client: &Client, name: &str) -> Result<Vec<Like>> {
    get_paged_json_data(client, &craft_users_url(name, LIKES_JSON)?)
}

pub fn get_user_followers(client: &Client, name: &str) -> Result<Vec<Follower>> {
    get_paged_json_data(client, &craft_users_url(name, FOLLOWERS_JSON)?)
}

pub fn get_user_followings(client: &Client, name: &str) -> Result<Vec<Follower>> {
    get_paged_json_data(client, &craft_users_url(name, FOLLOWINGS_JSON)?)
}

pub fn get_user_about(client: &Client, name: &str) -> Result<Profile> {
    let url = craft_users_url(name, PROFILE_JSON)?;
    let mut response = client.get(&url).send()?;
    Ok(response.json::<Profile>()?)
}