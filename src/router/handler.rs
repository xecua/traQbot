use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::*;

use super::receiver::*;
use super::guards::*;
use super::super::database::schema::Database;
use super::super::database::operation::*;


#[get("/")]
pub fn index() -> &'static str {
    "test ok"
}

// PING, JOINED, LEFTイベント
#[post("/")]
pub fn empty(header: Header) -> Status {
    let Header(event, _) = header;
    match &*event {
        "PING" | "JOINED" | "LEFT" => Status::NoContent,
        _ => Status::BadRequest
    }
} 

// #[post("/", data="<data>")]
// pub fn message(data: Json)