table! {
    channels (id) {
        id -> Integer,
        channel_id -> Varchar,
    }
}

table! {
    songs (id) {
        id -> Integer,
        title -> Varchar,
        difficulty -> Integer,
        level_val -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    songs,
);
