use diesel::prelude::*;
use super::schema::{songs, aprilfools};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub enum Difficulty {
    PAST,
    PRESENT,
    FUTURE,
    APRIL
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub difficulty: Difficulty,
    pub level_val: Option<i32>
}


#[derive(Insertable)]
#[table_name="songs"]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub difficulty: &'a Difficulty,
    pub level_val: &'a i32
}

