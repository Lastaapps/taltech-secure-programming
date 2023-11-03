use diesel::prelude::*;

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

