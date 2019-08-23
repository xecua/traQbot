use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::*;
use regex::Regex;

use super::receiver::*;

lazy_static!{
    static ref TIME_STAMP_PATTERN: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d*Z$").unwrap();
}

#[get("/")]
pub fn index() -> &'static str {
    "test ok"
}

// PINGイベント
#[post("/", data="<data>", rank=1)]
pub fn ping_event(data: Json<Ping>) -> Status {
    if TIME_STAMP_PATTERN.is_match(&data.eventTime) {
        Status::NoContent
    }
    else {
        Status::BadRequest
    }
} 

// JOINEDイベント
#[post("/", data="<data>", rank=2)]
pub fn joined_left_event(data: Json<Channelevent>) -> &'static str {
    "channel event ok"
}
