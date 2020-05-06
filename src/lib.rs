#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{NewUser, NewUserSession, NewTransaction}; //{NewUser, User, Transaction};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_user_session(
    conn: &SqliteConnection,
    user_id: i32,
    token: String,

) {
    use schema::user_sessions;

    let new_user_session = NewUserSession {
        user_id,
        token,
    };

    diesel::insert_into(user_sessions::table)
        .values(&new_user_session)
        .execute(conn)
        .expect("Error saving new user session");
}

pub fn create_new_user(
    conn: &SqliteConnection,
    nickname: String,
    password: String,
    email: String,
) {
    use schema::users;

    let new_user = NewUser {
        nickname,
        password,
        email,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

}

pub fn create_new_transaction(
    conn: &SqliteConnection,
    user_id: i32,
    date_transaction: String,
    sell_amount: f32,
    sell_currency: String,
    buy_amount: f32,
    buy_currency: String,
    price_for_one: f32,
) {
    use schema::transactions;

    let new_transaction = NewTransaction {
        user_id,
        date_transaction,
        sell_amount,
        sell_currency,
        buy_amount,
        buy_currency,
        price_for_one,
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .execute(conn)
        .expect("Error saving new transaction");

}