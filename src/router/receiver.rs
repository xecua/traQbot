#![allow(non_snake_case)]
#![allow(dead_code)]
use rocket::http::RawStr;
use rocket::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    displayName: String,
    iconId: String,
    bot: bool,
}

#[derive(Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    path: String,
    parentId: String,
    creator: User,
    createdAt: String,
    updatedAt: String,
}

#[derive(Deserialize)]
pub struct Embed {
    raw: String,
    r#type: String,
    id: String,
}

#[derive(Deserialize)]
pub struct Message {
    id: String,
    pub user: User,
    pub channelId: String,
    text: String,
    pub plainText: String,
    embedded: Vec<Embed>,
    createdAt: String,
    updatedAt: String,
}

#[derive(Deserialize)]
pub struct Ping {
    eventTime: String,
}

#[derive(Deserialize)]
pub struct JoinLeft {
    eventTime: String,
    channel: Channel,
}

#[derive(Deserialize)]
pub struct MessageCreated {
    eventTime: String,
    pub message: Message,
}
