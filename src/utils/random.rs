use crate::database::Database;
use crate::database::models::Difficulty;

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

pub fn random_choice(terms: Vec<String>, user_name: &String, conn: &Database) -> String {
    use crate::constants::arcaea::{DIFFICULTY, ODAI};
    use crate::database::operation::{get_random_one, SongWithDif};
    use super::make_mention;
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
                user_name, song.title, song.difficulty, task
            )
        }
        Err(e) => format!("@{} {}", user_name, e),
    }
}
