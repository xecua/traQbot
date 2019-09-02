use super::Database;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_contrib::databases::diesel::MysqlConnection;
use log::{error, warn, info, debug};

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
