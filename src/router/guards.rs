// リクエストガード
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;

lazy_static! {
    static ref verification_code: String = std::env::var("BOT_VERIFICATION_CODE").unwrap();
}

#[derive(Debug)]
pub enum HeaderError {
    FieldNotFound,
    Unauthorized,
    UnknownEventName
}

// ヘッダを検証
pub struct Header(pub String, pub String);
impl<'a, 'r> FromRequest<'a, 'r> for Header {
    type Error = HeaderError;
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-traq-bot-token") {
            Some(ref tok) if tok == &*verification_code => match request.headers().get_one("x-traq-bot-event") {
                Some(evt) => Outcome::Success(Header(evt.to_string(), tok.to_string())),
                None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
            },
            Some(_) => Outcome::Failure((Status::Unauthorized, HeaderError::Unauthorized)),
            None => Outcome::Failure((Status::BadRequest, HeaderError::FieldNotFound))
        }
    }
}