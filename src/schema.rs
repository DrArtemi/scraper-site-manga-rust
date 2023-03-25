// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Integer,
        url -> Text,
        number -> Text,
        date -> Text,
    }
}

diesel::table! {
    comics (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        cover_url -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chapters,
    comics,
);
