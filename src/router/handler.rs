use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::*;
use rocket_contrib::json::*;

use super::guards::*;
use super::receiver::*;
use crate::database::Database;
use crate::utils::command::*;
use crate::utils::sender::send_message;
use crate::utils::*;

use log::{debug, error, info, warn};

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
    if let Err(e) = match parse_command(&data.message.plainText) {
        Some(Command::Help) => {
            use crate::constants::HELP_TEXT;
            send_message(&data.message.channelId, HELP_TEXT.to_string())
        }
        Some(Command::Random(terms)) => {
            use crate::utils::random::random_choice;
            send_message(
                &data.message.channelId,
                random_choice(terms, &data.message.user.name, &conn),
            )
        }
        Some(Command::Stamp(num, terms)) => {
            use crate::utils::stamp::stamp;
            send_message(
                &data.message.channelId,
                stamp(num, terms, &data.message.user.name),
            )
        }
        Some(Command::Omikuji) => {
            if let Err(e) = send_message(
                &data.message.channelId,
                String::from("おみくじ代行サービス代行サービスです！"),
            ) {
                Err(e)
            } else {
                send_message(
                    &data.message.channelId,
                    String::from("@BOT_aya_se おみくじ代行サービス"),
                )
            }
        }
        None => Ok(()),
    } {
        error!("{}", e);
        Status::InternalServerError
    } else {
        Status::NoContent
    }
}
