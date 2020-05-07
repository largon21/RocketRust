use super::schema::*;
use serde_derive::Serialize;

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

//---------TABLE transactions-------------
#[derive(Serialize)]
#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub date_transaction: String,
    pub sell_amount: f32,
    pub sell_currency: String,
    pub buy_amount: f32,
    pub buy_currency: String,
    pub price_for_one: f32,
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction {
    pub user_id: i32,
    pub date_transaction: String,
    pub sell_amount: f32,
    pub sell_currency: String,
    pub buy_amount: f32,
    pub buy_currency: String,
    pub price_for_one: f32,
}