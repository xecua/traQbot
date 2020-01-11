use crate::database::Database;
use crate::database::models::Channel;
use super::sender::send_message;

pub fn join_channel(channel: &str, conn: &Database) -> Result<(), String> {
    use super::sender::join_channel_request;
    use crate::database::operation::insert_channel;

    if let Err(e) = insert_channel(&*conn, channel) {
        return Err(e.to_string());
    }

    match join_channel_request(channel) {
        Err(e) => {
            if let Err(er) = send_message(channel, format!("参加に失敗しました...\n@xecua {}", e)) {
                return Err(er.to_string());
            }
            return Err(e.to_string());
        }
        _ => ()
    }

    send_message(channel, String::from("参加しました！"))
}

pub fn leave_channel(channel: &str, conn: &Database) -> Result<(), String> {
    use super::sender::leave_channel_request;
    use crate::database::operation::delete_channel;

    if let Err(e) = leave_channel_request(channel) {
        if let Err(er) = send_message(channel, format!("離脱に失敗しました...\n@xecua {}", e)) {
            return Err(er.to_string());
        }
        return Err(e.to_string());
    }

    if let Err(e) = delete_channel(&*conn, channel) {
        return Err(e.to_string());
    }

    send_message(channel, String::from("離脱しました"))
}
