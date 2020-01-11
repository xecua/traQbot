pub mod command;
pub mod random;
pub mod sender;
pub mod stamp;

pub fn make_mention(name: &String, id: &String) -> String {
    format!(
        "!{{\"type\": \"user\", \"raw\": \"@{}\", \"id\": \"{}\"}}",
        name, id
    )
}
