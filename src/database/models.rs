use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub past_difficulty: i32,
    pub present_difficulty: i32,
    pub future_difficulty: i32
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Aprilfool {
    pub id: i32,
    pub title: String
}