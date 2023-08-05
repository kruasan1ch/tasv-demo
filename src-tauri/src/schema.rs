// @generated automatically by Diesel CLI.

table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        done -> Bool,
    }
}
