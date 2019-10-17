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

pub fn get_random_one_with_option(conn: &MysqlConnection, option: &RandomOption) -> Result<SongWithDif, Error> {
    use super::schema::songs::dsl::*;
    use super::models::Song;
    use rand::seq::SliceRandom;
    
    let res = songs.filter(past_difficulty.eq_any(option.levels.clone())
                            .or(present_difficulty.eq_any(option.levels.clone())
                            .or(future_difficulty.eq_any(option.levels.clone())))
                            ).load::<Song>(conn)?;
    
    let mut rng = rand::thread_rng();
    
    match res.choose(&mut rng) {
        Some(song) => {
            let mut d: Vec<String> = Vec::new();
            if option.levels.contains(&song.past_difficulty.unwrap()) {
                d.push(String::from("PAST"));
            }
            if option.levels.contains(&song.present_difficulty.unwrap()) {
                d.push(String::from("PRESENT"));
            }
            if option.levels.contains(&song.future_difficulty.unwrap()) {
                d.push(String::from("FUTURE"));
            }
            
            match d.choose(&mut rng) {
                Some(dif) => {
                    Ok(SongWithDif {
                        title: song.title.clone(),
                        difficulty: dif.to_string()
                    })
                }
                None => Err(Error::NotFound)
            }
        }
        None => Err(Error::NotFound)
    }
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
