#[allow(non_snake_case)]
use serde_derive::Deserialize;
use rocket::*;
use rocket::http::RawStr;


#[derive(Deserialize)]
pub struct User {
    id: String,
    name: String,
    displayName: String,
    iconId: String,
    bot: bool
}

#[derive(Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    path: String,
    parentId: String,
    creator: User,
    createdAt: String,
    updatedAt: String
}

#[derive(Deserialize)]
pub struct Embed {
    raw: String,
    r#type: String,
    id: String
}

#[derive(Deserialize)]
pub struct Message {
    id: String,
    user: User,
    channelId: String,
    text: String,
    plainText: String,
    embedded: Vec<Embed>,
    createdAt: String,
    updatedAt: String
}

#[derive(Deserialize)]
pub struct MessageCreated {
    eventTime: String,
    message: Message
}