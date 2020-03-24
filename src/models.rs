use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub token: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser{
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub token: i32,
}