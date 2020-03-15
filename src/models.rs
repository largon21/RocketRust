use schema::*;

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users_sessions"]
pub struct NewUserSession {
    pub user_id: i64,
    pub token: String,
}
