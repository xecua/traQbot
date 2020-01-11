use super::sender::get_stamp_name_list;
use crate::constants::BASE_URL;
use rand::Rng;
use reqwest::header::AUTHORIZATION;

pub fn stamp(num: usize, terms: Vec<String>, user_name: &String) -> String {
    if num == 0 {
        return format!("@{}", user_name);
    }

    let stamps = if terms.len() == 0 {
        match get_stamp_name_list() {
            Ok(s) => s,
            Err(e) => return e
        }
    } else {
        terms
    };

    let mut items: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        items.push(format!(":{}:", stamps[rng.gen_range(0, stamps.len())]));
    }
    format!("{}", items.join(""))
}
