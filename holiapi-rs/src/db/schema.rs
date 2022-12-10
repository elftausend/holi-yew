diesel::table! {
    users (user_id) {
        user_id -> Text,
        username -> Text,
        entry_info -> Text,
        flag_count -> Integer,
        banned -> Integer,
        class -> Text,
    }
}
