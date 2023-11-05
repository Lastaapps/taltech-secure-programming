use diesel::prelude::*;
use time::{OffsetDateTime, PrimitiveDateTime};

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

// Ciphers
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

