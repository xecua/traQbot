use super::super::database::Database;
use super::receiver::*;

pub enum Command {
    Help,
    Random(Vec<String>),
}

// コマンドがあればそれを↑のEnum形式で、なければNoneを返す
pub fn parse_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    let mut terms = plain_text.split_whitespace();
    match terms.next() {
        Some("@BOT_xecua_odai") => {
            let command = terms.next();
            if command == None {
                return None;
            }
            let command = command.unwrap();
            if command.eq_ignore_ascii_case("help") || command.eq_ignore_ascii_case("/help") {
                Some(Help)
            } else if command.eq_ignore_ascii_case("random")
                || command.eq_ignore_ascii_case("/random") {
                Some(Random(terms.map(|x| x.to_string()).collect()))
            } else {
                None
            }
        }
        Some(command) => {
            if command.eq_ignore_ascii_case("/help") {
                Some(Help)
            } else if command.eq_ignore_ascii_case("/random") {
                Some(Random(terms.map(|x| x.to_string()).collect()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
スラッシュコマンド形式での投稿を行うと該当する内容を実行します(リプライしてもしなくてもいいです あとリプライのときはスラッシュなくてもいいです)
+ help : このヘルプを出します
+ random : 全曲全譜面から適当にお題を出します
## :shiyourei_shi::shiyourei_you::shiyourei_rei:
+ `@BOT_xecua_odai help` と投稿すると、ヘルプを出します
+ `/random` と投稿すると、適当にお題を出します
## 注意点
+ このBotは現状このチャンネル(#gps/times/xecua)しか監視していませんし@BOT_toki_testみたいに有能じゃないのでなんかしたらそこも見始めるとかないです
"#;

pub struct RandomOption {
    pub levels: Vec<i32>
    pub difficulties: Vec<i32>
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
    use super::super::constants::arcaea::{DIFFICULTY, ODAI};
    use super::super::database::operation::get_random_one, get_random_one_with_option;
    use super::super::utils::make_mention;
    use rand::seq::SliceRandom;

    match terms.next() {
        // std::vec::Vec::contains?
        Some(s) => {
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
                    options.difficulties.push("Past");
                } else if "present".eq_ignore_ascii_case(&option) || "prs".eq_ignore_ascii_case(&option) {
                    options.difficulties.push("Present");
                } else if "future".eq_ignore_ascii_case(&option) || "ftr".eq_ignore_ascii_case(&option) {
                    options.difficulties.push("Future");
                }
            }
            
            if let Ok(title) = get_random_one_with_option(&*conn, &options) {

            }
            
        }
        None => {
            if let Ok(title) = get_random_one(&*conn) {
                let mut rng = rand::thread_rng();
                let dif = DIFFICULTY.choose(&mut rng).unwrap();
                let task = ODAI.choose(&mut rng).unwrap();

                format!(
                    "{} {} {}を{}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    title,
                    dif,
                    task
                )
            } else {
                format!(
                    "{} {}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    String::from("曲が入ってねぇ")
                )
            }
        }
    }
}
