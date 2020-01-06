#![allow(non_snake_case)]
#![allow(dead_code)]
use super::super::database::Database;
use super::receiver::*;
use super::BASE_URL;
use serde_derive::Deserialize;

#[derive(PartialEq, Debug)] // for debug/test
pub enum Command {
    Help,
    Random(Vec<String>),
    Stamp(usize, Vec<String>),
    Omikuji,
}

fn comma_split<T: std::iter::Iterator<Item = String>>(iter: T) -> Vec<String> {
    let mut res = Vec::new();
    for i in iter {
        for c in i.split(':').filter(|x| !x.is_empty()) {
            res.push(c.to_string());
        }
    }
    res
}

pub fn parse_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    let mut terms = plain_text.split_whitespace().map(|x| x.to_lowercase());
    let command = terms.next();
    if command.is_none() {
        return None;
    }

    let mut command = command.unwrap();
    if command.as_str() == "@bot_xecua_odai" {
        match terms.next() {
            Some(c) => {
                command = c;
            }
            None => {
                return None;
            }
        }
    }

    match command.as_str() {
        "/help" => Some(Help),
        "/random" => Some(Random(terms.collect())),
        "/stamp" => match terms.clone().next() {
            // clone `terms` not to consume original
            Some(s) => match s.parse::<usize>() {
                Ok(n) => Some(Stamp(n, comma_split(terms.skip(1)))),
                Err(_) => Some(Stamp(1, comma_split(terms))),
            },
            None => Some(Stamp(1, Vec::new())),
        },
        "/omikuji" => Some(Omikuji),
        _ => None,
    }
}

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
スラッシュコマンド形式での#gps/times/xecuaへの投稿、あるいはこのbotへのメンションを行うと該当する内容を実行します
+ `/help` : このヘルプを出します
+ `/random [difficulty|level]` : 全曲全譜面から適当にお題を出します
  + `difficulty`に難易度(PAST/PST, PRESENT/PRS, FUTURE/FTR)を空白区切りで指定すると、その中からのみ出題します
  + `level`にレベル値(1~9,9+,10)を指定すると、その中からのみ出題します
+ `/stamp [n] [stamp_list]`: n個のスタンプをランダムで召喚します 省略した場合n=1です stamp_listは空白区切りでスタンプ名を置くとそこからn個選択します(ちなみに存在するかはチェックしません) 省略するとtraQから引っ張ってきます
+ /omikuji おみくじ代行サービス代行サービスです
"#;

use super::super::database::models::Difficulty;
pub struct RandomOption {
    pub levels: Vec<i32>,
    pub difficulties: Vec<Difficulty>,
}

impl RandomOption {
    pub fn new() -> RandomOption {
        use std::vec::Vec;

        RandomOption {
            levels: Vec::new(),
            difficulties: Vec::new(),
        }
    }
}

pub fn random_choice(terms: Vec<String>, data: &MessageCreated, conn: &Database) -> String {
    use super::super::constants::arcaea::{DIFFICULTY, ODAI};
    use super::super::database::operation::{get_random_one, SongWithDif};
    use super::super::utils::make_mention;
    use rand::seq::SliceRandom;

    let mut options = RandomOption::new();

    for option in terms {
        if option == "1" {
            options.levels.push(1)
        } else if option == "2" {
            options.levels.push(2);
        } else if option == "3" {
            options.levels.push(3);
        } else if option == "4" {
            options.levels.push(4);
        } else if option == "5" {
            options.levels.push(5);
        } else if option == "6" {
            options.levels.push(6);
        } else if option == "7" {
            options.levels.push(7);
        } else if option == "8" {
            options.levels.push(8);
        } else if option == "9" {
            options.levels.push(9);
        } else if option == "9+" {
            options.levels.push(10);
        } else if option == "10" {
            options.levels.push(11);
        } else if "past".eq_ignore_ascii_case(&option) || "pst".eq_ignore_ascii_case(&option) {
            options.difficulties.push(Difficulty::PAST);
        } else if "present".eq_ignore_ascii_case(&option) || "prs".eq_ignore_ascii_case(&option) {
            options.difficulties.push(Difficulty::PRESENT);
        } else if "future".eq_ignore_ascii_case(&option) || "ftr".eq_ignore_ascii_case(&option) {
            options.difficulties.push(Difficulty::FUTURE);
        }
    }

    match get_random_one(&*conn, options) {
        Ok(song) => {
            let task = ODAI.choose(&mut rand::thread_rng()).unwrap();

            format!(
                "@{} 『{}』 {}を{}",
                &data.message.user.name, song.title, song.difficulty, task
            )
        }
        Err(e) => format!("@{} {}", &data.message.user.name, e),
    }
}

#[derive(Deserialize)]
struct StampsResponse {
    id: String,
    pub name: String,
    creatorId: String,
    fileId: String,
    createdAt: String,
    updatedAt: String,
}

lazy_static! {
    static ref ACCESS_TOKEN: String = std::env::var("BOT_ACCESS_TOKEN").unwrap();
}

pub fn stamp(num: usize, terms: Vec<String>, data: &MessageCreated) -> String {
    use reqwest::header::AUTHORIZATION;

    if num == 0 {
        return format!("@{}", &data.message.user.name);
    }

    let stamps: Vec<String>;
    if terms.len() == 0 {
        let endpoint = reqwest::Url::parse(&format!("{}/stamps", BASE_URL)).unwrap();
        let client = reqwest::Client::new();
        let res = client
            .get(endpoint)
            .header(AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
            .send();
        let j: Vec<StampsResponse>;
        match res {
            Ok(mut resp) => match resp.json() {
                Ok(json) => j = json,
                Err(e) => return format!("@{} {}", &data.message.user.name, e),
            },
            Err(e) => return format!("@{} {}", &data.message.user.name, e),
        }
        stamps = j.into_iter().map(|s| format!("{}", s.name)).collect();
    } else {
        stamps = terms;
    }
    use rand::Rng;
    let mut items: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        items.push(format!(":{}:", stamps[rng.gen_range(0, stamps.len())]));
    }
    format!("{}", items.join(""))
}
