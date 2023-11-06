use diesel::prelude::*;
use time::PrimitiveDateTime;

// User
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateUserDto {
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LoginUserDto {
    pub username: String,
    pub password_hash: String,
    pub deleted: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDeleteCheckDto {
    pub deleted: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserIdDto {
    pub id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserUsernameDto {
    pub username: String,
}

// Ciphers - Ceasar
#[derive(Insertable)]
#[diesel(table_name = crate::schema::ceasar)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertCeasarDto {
    pub user_id: i32,
    pub data: String,
    pub shift: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ceasar)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetCeasarDto {
    pub id: i32,
    pub data: String,
    pub shift: i32,
    pub created: PrimitiveDateTime,
    pub updated: PrimitiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ceasar)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetCeasarInternalsDto {
    pub data: String,
    pub shift: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ceasar)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetCeasarForRecoveryDto {
    pub id: i32,
    pub data: String,
    pub created: PrimitiveDateTime,
    pub updated: PrimitiveDateTime,
}

// Ciphers - Vigenere
#[derive(Insertable)]
#[diesel(table_name = crate::schema::vigenere)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertVigenereDto {
    pub user_id: i32,
    pub data: String,
    pub key: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vigenere)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetVigenereDto {
    pub id: i32,
    pub data: String,
    pub key: String,
    pub created: PrimitiveDateTime,
    pub updated: PrimitiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vigenere)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetVigenereInternalsDto {
    pub data: String,
    pub key: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vigenere)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GetVigenereForRecoveryDto {
    pub id: i32,
    pub data: String,
    pub created: PrimitiveDateTime,
    pub updated: PrimitiveDateTime,
}
