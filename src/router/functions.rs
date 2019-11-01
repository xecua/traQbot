use super::super::database::Database;
use super::receiver::*;

#[derive(PartialEq,Debug)] // for debug/test
pub enum Command {
    Help,
    Random(Vec<String>),
}

// コマンドがあればそれを↑のEnum形式で、なければNoneを返す
pub fn parse_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    let mut terms = plain_text.split_whitespace().map(|x| x.to_lowercase());     //ケースインセンシティブ化　全て小文字に直してから処理しています
    let command = terms.next();
    if command.is_none() {
        return None;
    }

    let mut command = command.unwrap();
    if command.as_str() == "@bot_xecua_odai" {
        match terms.next() {
            Some(c) => { command = c; }
            None => { return None; }
        }
    }

    match command.as_str() {
        "/help" => Some(Help),
        "/random" => Some(Random(terms.map(|x| x.to_string()).collect())),
        _ => None
    }
}

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
スラッシュコマンド形式での#gps/times/xecuaへの投稿、あるいはこのbotへのメンションを行うと該当する内容を実行します
+ `/help` : このヘルプを出します
+ `/random` : 全曲全譜面から適当にお題を出します
  + さらに、スペース区切りで難易度値(1~10,9+)を指定すると、その中からのみ出題します
  + また、スペース区切りで難易度(PAST/PST, PRESENT/PRS, FUTURE/FTR)を指定すると、その中からのみ出題します
## 最近のアップデート: v1.1.4
+ 反応の方式を変更(**リプライにおいてもスラッシュを必要とするようにしました**、気分です)
## 直近のアップデート: v1.2.0
+ 難易度指定に対応
"#;

use super::super::database::models::Difficulty;
pub struct RandomOption {
    pub levels: Vec<i32>,
    pub difficulties: Vec<Difficulty>
}

impl RandomOption {
    pub fn new() -> RandomOption {
        use std::vec::Vec;

        RandomOption {
            levels: Vec::new(),
            difficulties: Vec::new()
        }
    }
}

pub fn random_choice(terms: Vec<String>, data: &MessageCreated, conn: &Database) -> String {
    use super::super::constants::arcaea::{DIFFICULTY,ODAI};
    use super::super::database::operation::{get_random_one,SongWithDif};
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
                &data.message.user.name,
                song.title,
                song.difficulty,
                task
            )
        }
        Err(e) => {
            format!(
                "@{} {}",
                &data.message.user.name,
                e
            )
        }
    }
}
