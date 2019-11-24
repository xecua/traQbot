use reqwest::header::AUTHORIZATION;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::*;
use rocket_contrib::json::*;
use std::collections::HashMap;

use super::super::database::Database;
use super::guards::*;
use super::receiver::*;
use super::BASE_URL;
use log::{debug, error, info, warn};

lazy_static! {
    static ref ACCESS_TOKEN: String = std::env::var("BOT_ACCESS_TOKEN").unwrap();
}

#[get("/")]
pub fn index() -> std::io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// PING
#[post("/", data = "<_data>", rank = 1)]
pub fn ping(_header: Header, _ping_header: PingHeader, _data: Json<Ping>) -> Status {
    Status::NoContent
}

// JOINED, LEFT
#[post("/", data = "<_data>", rank = 2)]
pub fn join_left(
    _header: Header,
    _join_left_header: JoinLeftHeader,
    _data: Json<JoinLeft>,
) -> Status {
    Status::NoContent
}

// MESSAGE_CREATED
#[post("/", data = "<data>", rank = 3)]
pub fn message(
    _header: Header,
    _message_header: MessageHeader,
    data: Json<MessageCreated>,
    conn: Database,
) -> Status {
    use super::functions::*;

    // 投稿するメッセージ
    let mut body = HashMap::new();
    let command = parse_command(&data.message.plainText);
    match command {
        Some(Command::Help) => {
            body.insert("text", HELP_TEXT.to_string());
        }
        Some(Command::Random(terms)) => {
            body.insert("text", random_choice(terms, &data, &conn));
        }
        Some(Command::Stamp(num, terms)) => {
            body.insert("text", stamp(num, terms, &data));
        }
        None => {
            return Status::NoContent;
        }
    }
    if cfg!(debug_assertions) {
        debug!("{:?}", body);
    } else {
        // チャンネル
        let channel_id = data.message.channelId.clone();
        let endpoint =
            reqwest::Url::parse(&format!("{}/channels/{}/messages", BASE_URL, channel_id)).unwrap();

        // 投げる
        let client = reqwest::Client::new();
        let res = client
            .post(endpoint)
            .query(&[("embed", "1")])
            .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
            .json(&body)
            .send();

        match res {
            Ok(resp) => info!(
                "Sending was succeeded. Here's response code: {}",
                resp.status().as_u16()
            ),
            Err(_) => warn!("Failed to post"),
        };
    }
    Status::NoContent
}
