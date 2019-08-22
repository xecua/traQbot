use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::*;

use super::receiver::*;

#[get("/")]
pub fn index() -> &'static str {
    "test ok"
}

// PINGイベント
#[post("/", data="<data>", rank=1)]
pub fn ping_event(data: Json<Ping>) -> Status {
    Status::NoContent
} 

// JOINEDイベント
#[post("/", data="<data>", rank=2)]
pub fn joined_left_event(data: Json<Channelevent>) -> &'static str {
    "channel event ok"
}
