use super::schema::*;

//---------TABLE users-------------
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser{
    pub nickname: String,
    pub password: String,
    pub email: String,
}

//---------TABLE user_sessions-------------
#[derive(Queryable)]
pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

#[derive(Insertable)]
#[table_name="user_sessions"]
pub struct NewUserSession {
    pub user_id: i32,
    pub token: String,
}