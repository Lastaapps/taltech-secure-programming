// @generated automatically by Diesel CLI.

diesel::table! {
    ceasar (id) {
        id -> Nullable<Integer>,
        shift -> Integer,
        data -> Text,
        deleted -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    roles_users (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password_hash -> Text,
        salt -> Text,
        deleted -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vigenere (id) {
        id -> Nullable<Integer>,
        key -> Text,
        data -> Text,
        deleted -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(roles_users -> roles (role_id));
diesel::joinable!(roles_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ceasar,
    roles,
    roles_users,
    users,
    vigenere,
);
