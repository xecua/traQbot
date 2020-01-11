#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::constants::BASE_URL;
use log::{debug, error, info, warn};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use reqwest::Url;
use std::collections::HashMap;

#[derive(Deserialize)]
struct StampsResponse {
    id: String,
    pub name: String,
    creatorId: String,
    fileId: String,
    createdAt: String,
    updatedAt: String,
}

lazy_static! {
    static ref ACCESS_TOKEN: String = std::env::var("BOT_ACCESS_TOKEN").unwrap();
    static ref BOT_ID: String = std::env::var("BOT_ID").unwrap();
    static ref BOT_INSTALL_CODE: String = std::env::var("BOT_INSTALL_CODE").unwrap();
}

#[must_use]
#[cfg(debug_assertions)]
pub fn send_message(_: &str, text: String) -> Result<(), String> {
    debug!("{}", text);
    Ok(())
}

#[must_use]
#[cfg(not(debug_assertions))]
pub fn send_message(channel_id: &str, text: String) -> Result<(), String> {
    let endpoint = Url::parse(&format!("{}/channels/{}/messages", BASE_URL, channel_id)).unwrap();
    let mut body = HashMap::new();
    body.insert("text", text);

    // 投げる
    let client = Client::new();
    let res = client
        .post(endpoint)
        .query(&[("embed", "1")])
        .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .json(&body)
        .send();

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[must_use]
fn get_stamp_json() -> Result<Vec<StampsResponse>, reqwest::Error> {
    let endpoint = Url::parse(&format!("{}/stamps", BASE_URL)).unwrap();

    Client::new()
        .get(endpoint)
        .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .send()?.json()
}

#[must_use]
pub fn get_stamp_name_list() -> Result<Vec<String>, String> {
    match get_stamp_json() {
        Ok(j) => Ok(j.into_iter().map(|s| format!("{}", s.name)).collect()),
        Err(e) => Err(e.to_string())
    }
}

pub fn join_channel_request(channel: &str) -> Result<(), String> {
    let endpoint = Url::parse(&format!("{}/channels/{}/bots", BASE_URL, channel)).unwrap();
    let mut body = HashMap::new();
    body.insert("code", &*BOT_INSTALL_CODE);

    match Client::new()
            .post(endpoint)
            .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
            .json(&body)
            .send() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn leave_channel_request(channel: &str) -> Result<(), String> {
    let endpoint = Url::parse(&format!("{}/channels/{}/bots/{}", BASE_URL, channel, &*BOT_ID)).unwrap();

    match Client::new()
            .delete(endpoint)
            .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
            .send() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}