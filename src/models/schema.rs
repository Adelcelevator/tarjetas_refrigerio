// @generated automatically by Diesel CLI.
diesel::table! {
    likes (id) {
        id -> Int4,
        create_at -> Timestamp,
        tweet_id -> Int4,
    }
}

diesel::table! {
    tweets (id) {
        id -> Int4,
        create_at -> Timestamp,
        mensaje -> Text,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
