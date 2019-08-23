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

