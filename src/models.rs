use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub password: String,
}
