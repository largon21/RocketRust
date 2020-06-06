#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{NewUser, NewUserSession, NewTransaction, User}; //{  Transaction};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


//<-----------------user_session----------------->
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

//<-----------------user----------------->
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

pub fn update_user_nickname(
    conn: &SqliteConnection,
    username: String,
    new_username: String,
) {
    use schema::users::dsl::*;

    let updated_user =  users
        .filter(nickname.eq(username))
        .limit(1)
        .load::<User>(conn.clone())
        .expect("Error loading email");

    diesel::update(&updated_user[0])
        .set(nickname.eq(new_username))
        .execute(conn)
        .expect("Error new username");
}

pub fn update_user_email(
    conn: &SqliteConnection,
    username: String,
    new_email: String,
) {
    use schema::users::dsl::*;

    let updated_user =  users
        .filter(nickname.eq(username))
        .limit(1)
        .load::<User>(conn.clone())
        .expect("Error loading email");

    diesel::update(&updated_user[0])
        .set(email.eq(new_email))
        .execute(conn)
        .expect("Error new email");

}

pub fn update_user_password(
    conn: &SqliteConnection,
    username: String,
    new_password: String,
) {
    use schema::users::dsl::*;

    let updated_user =  users
        .filter(nickname.eq(username))
        .limit(1)
        .load::<User>(conn.clone())
        .expect("Error loading email");

    diesel::update(&updated_user[0])
        .set(password.eq(new_password))
        .execute(conn)
        .expect("Error new password");

}

//<-----------------transaction----------------->
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