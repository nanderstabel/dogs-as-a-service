// @generated automatically by Diesel CLI.

diesel::table! {
    dogs (id) {
        id -> Varchar,
        url -> Varchar,
        width -> Int4,
        height -> Int4,
        date -> Timestamp,
    }
}
