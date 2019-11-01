use super::Database;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_contrib::databases::diesel::MysqlConnection;
use log::{error, warn, info, debug};
use super::super::router::functions::RandomOption;

pub struct SongWithDif {
    pub title: String,
    pub difficulty: String
}

pub fn get_random_one(conn: &MysqlConnection, option: RandomOption) -> Result<SongWithDif, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;

    let res: Vec<Song>;
    if option.difficulties.len() > 0 && option.levels.len() > 0 {
        res = songs.filter(level_val.eq_any(option.levels)
                            .and(difficulty.eq_any(option.difficulties)))
                   .load::<Song>(conn)?;
    } else if option.difficulties.len() > 0 {
        res = songs.filter(difficulty.eq_any(option.difficulties))
                   .load::<Song>(conn)?;
    } else if option.levels.len() > 0 {
        res = songs.filter(level_val.eq_any(option.levels))
                   .load::<Song>(conn)?;
    } else {
        res = songs.load::<Song>(conn)?;
    }

    let mut rng = rand::thread_rng();

    match res.choose(&mut rng) {
        Some(song) => Ok(SongWithDif {
                title: song.title.clone(),
                difficulty: song.difficulty.to_string()
        }),
        None => Err(Error::NotFound)
    }
}

pub fn aprilfool(conn: &MysqlConnection) -> Result<String, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;

    let res = songs.filter(level_val.eq(0)).load::<Song>(conn)?;

    match res.choose(&mut rand::thread_rng()) {
        Some(song) => Ok(song.title.clone()),
        None => Err(Error::NotFound)
    }
}
