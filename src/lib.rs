#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewUser; //{NewUser, User};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_user_session(
    _conn: &SqliteConnection,
    user_id: i32,
    token: String,

)
{
    println!("{}, {}",user_id, token);
}

pub fn create_new_user(
    conn: &SqliteConnection,
    nickname: String,
    password: String,
    email: String,
    token: i32,
) {
    use schema::users;

    let new_user = NewUser {
        nickname,
        password,
        email,
        token,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

}