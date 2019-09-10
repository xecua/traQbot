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

pub fn get_random_one_with_option(conn: &MysqlConnection, option: &RandomOption) -> Result<String, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;
    
    // let res = songs.select()
}

pub fn aprilfool(conn: &MysqlConnection) -> Result<String, Error> {
    use super::schema::aprilfools::dsl::*;
    use super::models::Aprilfool;
    use rand::seq::SliceRandom;
    
    let res = aprilfools.load::<Aprilfool>(conn)?;
    
    match res.choose(&mut rand::thread_rng()) {
        Some(song) => Ok(song.title.clone()),
        None => Err(Error::NotFound)
    }
}
