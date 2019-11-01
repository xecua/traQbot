use super::Database;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_contrib::databases::diesel::MysqlConnection;
use log::{error, warn, info, debug};
use super::super::router::functions::RandomOption;

pub fn get_random_one(conn: &MysqlConnection) -> Result<String, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;

    let res = songs.load::<Song>(conn)?;
    
    match res.choose(&mut rand::thread_rng()) {
        Some(song) => Ok(song.title.clone()),
        None => Err(Error::NotFound)
    }
}

pub struct SongWithDif {
    pub title: String,
    pub difficulty: String
}

pub fn get_random_one_with_option(conn: &MysqlConnection, option: RandomOption) -> Result<SongWithDif, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;
    
    let res = songs.filter(level_val.eq_any(option.levels))
                   .load::<Song>(conn)?;
    
    let mut rng = rand::thread_rng();
    
    match res.choose(&mut rng) {
        Some(song) => Ok(SongWithDif {
                title: song.title.clone(),
                difficulty: song.level_val.to_string()
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
