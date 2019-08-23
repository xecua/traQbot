use super::Database;
use diesel::result::Error;

pub fn get_one(conn: &Database) -> Result<String, Error> {
    use schema::songs::dsl::*;

    let res = songs.load::<(i32, String, i32, i32, i32)>(&conn)?;
    
    match rand::seq::IteratorRandom(res) {
        Some(song) => song.title,
        None => ""
    }
}
