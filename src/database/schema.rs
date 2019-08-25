table! {
    aprilfools (id) {
        id -> Integer,
        title -> Varchar,
    }
}

table! {
    songs (id) {
        id -> Integer,
        title -> Varchar,
        past_difficulty -> Nullable<Integer>,
        present_difficulty -> Nullable<Integer>,
        future_difficulty -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    aprilfools,
    songs,
);
