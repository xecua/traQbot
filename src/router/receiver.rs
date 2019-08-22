use serde_derive::Deserialize;
use rocket::*;
use rocket::http::RawStr;

// FromFormValueとか使ってうまいことしたいけどできねぇ
#[derive(Deserialize)]
pub struct Ping {
    pub eventTime: String 
}

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
pub struct Channelevent {
    eventTime: String,
    channel: Channel
}
