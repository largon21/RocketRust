#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::NewUser;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_user(
    conn: &SqliteConnection,
    name: String,
    password: String
) {
    use schema::user;

    let new_user_session = NewUser{
        name,
        password
    };

    diesel::insert_into(user::table)
    .values(&new_user_session)
    .execute(conn)
    .expect("Error saving new session");
}
