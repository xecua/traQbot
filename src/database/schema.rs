use diesel::sql_types::*;

table! {
    songs {
        id -> Integer,
        title -> Varchar,
        past_difficulty -> Integer,
        present_difficulty -> Integer,
        future_difficulty -> Integer,
    }
}

table! {
    aprilfools {
        id -> Integer,
        title -> Varchar,
    }
}