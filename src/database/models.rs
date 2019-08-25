use diesel::prelude::*;
use super::schema::{songs, aprilfools};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub past_difficulty: Option<i32>,
    pub present_difficulty: Option<i32>,
    pub future_difficulty: Option<i32>
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Aprilfool {
    pub id: i32,
    pub title: String
}

#[derive(Insertable)]
#[table_name="songs"]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub past_difficulty: &'a i32,
    pub present_difficulty: &'a i32,
    pub future_difficulty: &'a i32
}

#[derive(Insertable)]
#[table_name="aprilfools"]
pub struct NewAprilfool<'a> {
    pub title: &'a str
}
