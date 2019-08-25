use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::*;
use std::collections::HashMap;
use reqwest::header::AUTHORIZATION;

use super::receiver::*;
use super::guards::*;
use super::super::database::Database;
use super::super::database::operation::*;
use log::{error, warn, info, debug};

lazy_static! {
    static ref ACCESS_TOKEN: String = std::env::var("BOT_ACCESS_TOKEN").unwrap();
}


#[get("/")]
pub fn index() -> &'static str {
    "おいす〜"
}

// PING, JOINED, LEFTイベント
#[post("/", rank=1)]
pub fn empty(header: Header) -> Status {
    let Header(event, _) = header;
    match &*event {
        "PING" | "JOINED" | "LEFT" => Status::NoContent,
        _ => Status::BadRequest
    }
} 

// MESSAGE_CREATED
#[post("/", data="<data>", rank=2)]
pub fn message(data: Json<Message>, header: Header, conn: Database) -> Status {
    // メッセージ。 仮おきでクソ課題
    let mut body = HashMap::new();
    body.insert("text", "Fracture Ray[FTR] Pure Memory");

    // チャンネル。仮おきでgps/times/xecua
    let channel_id = "55095d85-7066-4ac6-9a99-8655ef0cac92";
    let endpoint = reqwest::Url::parse(&format!("https://q.trap.jp/channels/{}/messages", channel_id)).unwrap();


    // 投げる
    let client = reqwest::Client::new();
    let res = client.post(endpoint)
        .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .json(&body)
        .send();
    match res {
        Ok(_) => info!("Succeeded in post"),
        Err(_) => warn!("Failed to post")
    };
    
    
    let Header(event, _) = header;
    match &*event {
        "MESSAGE_CREATED" => Status::NoContent,
        _ => Status::BadRequest
    }
}