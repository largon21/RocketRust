#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewUserSession, Users};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_user_session(conn: &SqliteConnection, user_id: i64, token: String) -> UserSession {
    use schema::user;

    let new_user_session = NewUserSession{
        user_id: user_id,
        token: token
    };

    diesel::insert_into(user::table).values(&new_user_session).execute(con).expect("Error saving new session");
    user::table.order(user::id.desc()).first(conn).unwrap()
}
