// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        user_id -> Int4,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, users,);
