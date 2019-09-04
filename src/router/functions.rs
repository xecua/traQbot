use super::super::database::Database;
use super::receiver::*;

pub enum Command {
    Help,
    Random
}

// コマンドがあればそれを↑のEnum形式で、なければNoneを返す
pub fn find_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    
    let mut terms = plain_text.split_whitespace();
    
    match terms.next() {
        Some("@BOT_xecua_odai") => {
            let command = terms.next();
            if command == None {
                return None;
            }
            
            let command = command.unwrap();
            if command.eq_ignore_ascii_case("help") {
                Some(Help)
            } else if command.eq_ignore_ascii_case("random") {
                Some(Random)
            } else {
                None
            }
        },
        Some(s) => {
            if s.chars().nth(0) == Some('/') {
                let command = &s[1..];
                if command.eq_ignore_ascii_case("help") {
                    Some(Help)
                } else if command.eq_ignore_ascii_case("random") {
                    Some(Random)
                } else {
                    None
                }
            }
            else {
                None
            }
        },
        _ => {
            None
        }
    }
}

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
コマンドをこのBotにリプライするか、スラッシュコマンド形式での投稿を行うと該当する内容を実行します
+ help : このヘルプを出します
+ random : 全曲全譜面から適当にお題を出します
## :shiyourei_shi::shiyourei_you::shiyourei_rei:
`@BOT_xecua_odai help` と投稿すると、ヘルプを出します
`/random` と投稿すると、適当にお題を出します
"#;

pub fn random_choice(data: &MessageCreated, conn: &Database) -> String {
    use super::super::database::operation::get_random_one;
    use super::super::utils::make_mention;
    use rand::seq::SliceRandom;
    use super::super::constants::arcaea::{DIFFICULTY, ODAI};

    if let Ok(title) = get_random_one(&*conn) {
        let mut rng = rand::thread_rng();
        let dif = DIFFICULTY.choose(&mut rng).unwrap();
        let task = ODAI.choose(&mut rng).unwrap();

       format!("{} {} {}を{}", make_mention(&data.message.user.name, &data.message.user.id), title, dif, task)
    } else {
        format!("{} {}", make_mention(&data.message.user.name, &data.message.user.id), String::from("曲が入ってねぇ"))
    }

}
