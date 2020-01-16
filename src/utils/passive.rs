use crate::constants::stamp::NAN_INDIA_STAMP_ID;
use crate::router::receiver::MessageCreated;
use crate::utils::sender::send_stamp;
use log::{debug, error, info, warn};
use regex::Regex;

pub fn do_passive_action(data: &MessageCreated) {
    if Regex::new(r"(ナン|なん)")
        .unwrap()
        .is_match(&data.message.plainText)
    {
        if let Err(e) = send_stamp(&data.message.channelId, NAN_INDIA_STAMP_ID) {
            error!("{}", e.to_string())
        }
    }
}
