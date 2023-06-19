// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        bodyone -> Text,
        bodytwo -> Text,
        imgone -> Text,
        imgtwo -> Text,
    }
}
