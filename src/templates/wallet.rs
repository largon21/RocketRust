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

#[derive(Serialize)]
pub struct DashboardDataContext {
    pub name: String,
    pub is_authenticated: bool,
    pub context_summary_transactions: Vec<SummaryTransactions>,
}

#[derive(Serialize)]
pub struct SummaryTransactions {
    coin: String,
    all_coins: f32, //
    all_spent_coins: f32, //in PLN
    current_price: f32,
    average_purchase_value: f32,
    
}

pub fn summarize_transactions(current_user_id: i32) -> Vec<SummaryTransactions> {

    let mut return_summary: Vec<SummaryTransactions> = Vec::new(); // return value as Vec<SummaryTransactions>

    let transactions: Vec<Transaction> = get_transactions_from_db(current_user_id);


    for transaction in transactions {
        // for buy currency
        if let Some(summary_transactions) = return_summary.iter().find(|&x| x.coin == transaction.buy_currency) {
            println!("omg");
        }
        else {
            let all_spent_coins: f32 = transaction.sell_amount*check_current_price_pln(transaction.sell_currency.clone()); // PLN
            let all_coins: f32 = transaction.buy_amount.clone(); //check wheter is not 0 !!!!!!!!!!!!!!!!!

            let foo = SummaryTransactions {
                coin: transaction.buy_currency.clone(),
                all_coins,
                all_spent_coins, // PLN
                current_price: check_current_price_pln(transaction.buy_currency.clone()),
                average_purchase_value: all_spent_coins/all_coins,
                
            };
            return_summary.push(foo);
        }

        // for sell currency
        if let Some(summary_transactions) = return_summary.iter().find(|&x| x.coin == transaction.sell_currency) {
            println!("omg");
        }
        else {
            let all_spent_coins: f32 = transaction.buy_amount*check_current_price_pln(transaction.buy_currency.clone()); // PLN
            let all_coins: f32 = (-1.0)*transaction.sell_amount.clone(); //check wheter is not 0 !!!!!!!!!!!!!!!!!

            let foo = SummaryTransactions {
                coin: transaction.sell_currency.clone(),
                all_coins,
                all_spent_coins, // PLN
                current_price: check_current_price_pln(transaction.sell_currency.clone()),
                average_purchase_value: all_spent_coins/all_coins,
                
            };
            return_summary.push(foo);
        }
    }

    return_summary
}

fn check_current_price_pln(currency_name: String) -> f32 {
    let return_value: f32 = 0.0;
    struct Currency {
        name: String,
        current_price_pln: f32,
    } 

    let btc = Currency { name: "BTC".to_string(), current_price_pln: 40000.0,};
    let bch = Currency { name: "BCH".to_string(), current_price_pln: 900.0,};
    let eth = Currency { name: "ETH".to_string(), current_price_pln: 850.0,};
    let ltc = Currency { name: "LTC".to_string(), current_price_pln: 170.0,};
    let xrp = Currency { name: "XRP".to_string(), current_price_pln: 0.80,};
    let pln = Currency { name: "PLN".to_string(), current_price_pln: 1.0,};
    let usd = Currency { name: "USD".to_string(), current_price_pln: 4.0,};
    let eur = Currency { name: "EUR".to_string(), current_price_pln: 4.4,};

    let currencies: Vec<Currency> = vec![btc, bch, eth, ltc, xrp, pln, usd, eur];

    for currency in currencies {
        if currency.name == currency_name {
            return currency.current_price_pln;
        }
    }
    return_value

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