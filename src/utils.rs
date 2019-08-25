pub fn make_mention(name: &String, id: &String) -> String {
    format!("!{{\"type\": \"user\", \"raw\": \"@{}\", \"id\": \"{}\"}}", name, id)
}