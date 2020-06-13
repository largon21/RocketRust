use hello_rocket::*;
use hello_rocket::models::Transaction;
use diesel::prelude::*;

#[derive(FromForm)]
pub struct TransactionForm {
    pub date_transaction: String,
    pub sell_amount: f32,
    pub sell_currency: String,
    pub buy_amount: f32,
    pub buy_currency: String,
}

#[derive(Serialize)]
pub struct TemplateContextWallet {
    pub name: String,
    pub is_authenticated: bool,
    pub context_transactions: Vec<Transaction>,
}

pub fn remove_transaction(active_user_id: i32, transaction_id: String) {
    use hello_rocket::schema::transactions::dsl::*;

    let transaction_id: i32 = transaction_id.parse().unwrap();
    let connection = establish_connection();

    let _deleted_transactions = diesel::
        delete(
            transactions
            .filter(user_id.eq(active_user_id))
            .filter(id.eq(transaction_id))
        )
        .execute(&connection)
        .expect("Error deleting transactions");
}

pub fn get_transactions_from_db(current_user_id: i32) -> Vec<Transaction> {
    use hello_rocket::schema::transactions::dsl::*;

    let connection = establish_connection();

    let results: Vec<Transaction> = transactions
        .filter(user_id.eq(current_user_id))
        .load::<Transaction>(&connection)
        .expect("Error loading sessions");
        
    //reverse to get from the newest to the oldest
    let results = results.into_iter().rev().collect();
    results
} 