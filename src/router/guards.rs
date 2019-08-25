// リクエストガード
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;

lazy_static! {
    static ref VERIFICATION_CODE: String = std::env::var("BOT_VERIFICATION_CODE").unwrap();
}

#[derive(Debug)]
pub enum HeaderError {
    FieldNotFound,
    Unauthorized,
    UnknownEventName
}

// トークンを検証
pub struct Header(String);
impl<'a, 'r> FromRequest<'a, 'r> for Header {
    type Error = HeaderError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-traq-bot-token") {
            Some(ref tok) if tok == &*VERIFICATION_CODE => Outcome::Success(Header(tok.to_string())),
            Some(_) => Outcome::Failure((Status::Unauthorized, HeaderError::Unauthorized)),
            None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
        }
    }
}

// イベント別のヘッダでハンドリング
pub struct PingHeader(String);
impl<'a, 'r> FromRequest<'a, 'r> for PingHeader {
    type Error = HeaderError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-traq-bot-event") {
            Some(evt) if evt == "PING" => Outcome::Success(PingHeader(evt.to_string())),
            Some(_) => Outcome::Forward(()),
            None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
        }
    }
}

pub struct JoinLeftHeader(String);
impl<'a, 'r> FromRequest<'a, 'r> for JoinLeftHeader {
    type Error = HeaderError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-traq-bot-event") {
            Some(evt) if evt == "JOINED" || evt == "LEFT" => Outcome::Success(JoinLeftHeader(evt.to_string())),
            Some(_) => Outcome::Forward(()),
            None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
        }
    }
}

pub struct MessageHeader(String);
impl<'a, 'r> FromRequest<'a, 'r> for MessageHeader {
    type Error = HeaderError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-traq-bot-event") {
            Some(evt) if evt == "MESSAGE_CREATED" => Outcome::Success(MessageHeader(evt.to_string())),
            Some(_) => Outcome::Forward(()),
            None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
        }
    }
}