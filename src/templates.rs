extern crate rocket_multipart_form_data;

use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, TextField, Repetition};

use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies}; 
use rocket::response::{Flash, Redirect};
use rocket::request::Form;

use hello_rocket::*;
use time::Duration;
use hello_rocket::models::{UserSession, Transaction, User};
use diesel::prelude::*;
use bcrypt::{DEFAULT_COST, hash, verify};



#[derive(Serialize)]
struct TemplateContext {
    name: String,
}



//<-----------------Login----------------->
struct Username(String);

#[derive(FromForm)]
pub struct UserLoginForm {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TemplateContextLogin {
    name: String,
    error_username: bool,
    error_password: bool,
}

fn get_password_hash_from_username_or_email(name: String) -> Result<String, std::io::Error> {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results;

    if name.contains("@") {
        results = users
            .filter(email.eq(name))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading email");
    }
    else {
        results = users
            .filter(nickname.eq(name))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading user");
    }
    
    
    if results.len() == 1 {
        return Ok(results[0].password.to_string());
    }
    else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no user found",
        ));
    }
}

fn get_user_id_from_username_or_email(name: String) -> Result<i32, std::io::Error> {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results;

    if name.contains("@") {
        results = users
            .filter(email.eq(name))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading email");
    }
    else {
        results = users
            .filter(nickname.eq(name))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading user");
    }
    
    if results.len() == 1 {
        return Ok(results[0].id);
    }
    else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no user found",
        ));
    }
}

fn generate_session_token(length: u8) -> Result<String, std::io::Error> {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    let strings: Vec<String> = bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    return Ok(strings.join(""));
}

fn login_validate_form(word: String) -> bool {
    if word.is_empty() {
        return true;
    }
    else {
        return false;
    }
}

#[post("/login", data = "<user>")]
pub fn login_post(mut cookies: Cookies, user: Form<UserLoginForm>) -> Result<Redirect, Template> {
    let error_username = login_validate_form(user.username.clone());
    let error_password = login_validate_form(user.password.clone());
    let mut username: String = format!("");


    if error_username || error_password {
        let name = "Error - empty space".to_string();
        let context = TemplateContextLogin {
            name, 
            error_username, 
            error_password
        };
        return Err(Template::render("login", &context));
    }
    else {
        match get_password_hash_from_username_or_email(user.username.clone()) {
            Ok(password_hash) => {
                println!("debug");
                match verify(&user.password, &password_hash) {
                    Ok(password_match) => {
                        if password_match {
                            match generate_session_token(64) {
                                Ok(session_token) => {
                                    let connection = establish_connection();
                                    match get_user_id_from_username_or_email(user.username.clone()) {
                                        Ok(user_id) => {
                                            create_new_user_session(&connection, user_id, session_token.clone());
                                            let mut c = Cookie::new("session_token", session_token);
                                            c.set_max_age(Duration::hours(24));
                                            cookies.add_private(c);
                                            return Ok(Redirect::to("/"));
                                        }
                                        Err(_) => {
                                            return Ok(Redirect::to("/"));
                                        }
                                    }
                                }
                                Err(_) => {
                                    let name = "Error login - token generation issue".to_string();
                                    let context = TemplateContextLogin {
                                        name, 
                                        error_username: false, 
                                        error_password: true
                                    };
                                    return Err(Template::render("login", &context));
                                }    
                            }
                        }
                        else {
                            let name = "Error login - password incorrect".to_string();
                            let context = TemplateContextLogin {
                                name, 
                                error_username: false, 
                                error_password: false
                            };
                            return Err(Template::render("login", &context));
                        }
                    }
                    Err(_) => {
                        let name = "Error login - verifying password incorrect".to_string();
                        let context = TemplateContextLogin {
                            name, 
                            error_username: false, 
                            error_password: false
                        };
                        return Err(Template::render("login", &context));
                    }
                }
            }
            Err(err) => {
                let name = format!("{}{}", "Error login - ", err);
                let context = TemplateContextLogin {
                    name, 
                    error_username: true, 
                    error_password: true
                };
                return Err(Template::render("login", &context));
            }
        }
    }
}

#[get("/login")]
pub fn login_get() -> Template {
    let context = TemplateContextLogin {
        name: "".to_string(), 
        error_username: false, 
        error_password: false
    };
    Template::render("login", &context)
}

//<-----------------Register----------------->
#[derive(FromForm)]
pub struct UserForm {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[derive(Serialize)]
struct TemplateContextRegister {
    name: String,
    error_username: bool,
    error_email: bool,
    error_password: bool,
    error_confirm_password: bool,

}

fn register_validate_username(username: String) -> bool {
    use hello_rocket::schema::users::dsl::*;

    if !username.is_empty() && username.len() >= 4 {
        let connection = establish_connection();
        let results = users
            .filter(nickname.eq(username))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading email");

        if results.len() == 0 {
            return false;
        }
        else {
            return true;
        }
        
    }
    else {
        return true;
    }
}

fn register_validate_email(user_email: String) -> bool {
    use hello_rocket::schema::users::dsl::*;

    if !user_email.is_empty() {
        let connection = establish_connection();
        let results = users
            .filter(email.eq(user_email))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading email");

        if results.len() == 0 {
            return false;
        }
        else {
            return true;
        }
    }
    else {
        return true;
    }
}

fn register_validate_password(user_password: String) -> bool {
    if !user_password.is_empty() && user_password.len() >= 4 {
        return false;
    }    
    else {
        return true;
    }
}

fn register_validate_confirm_password(str1: String, str2: String) -> bool {
    if str1 == str2 {
        return false;
    }    
    else {
        return true;
    }
}

#[post("/register", data = "<userdata>")]
pub fn register_post(userdata: Form<UserForm>) -> Result<Redirect, Template> {
    let error_username: bool = register_validate_username(userdata.username.clone());
    let error_email: bool = register_validate_email(userdata.email.clone());
    let error_password: bool = register_validate_password(userdata.password.clone()); 
    let error_confirm_password: bool 
        = register_validate_confirm_password(userdata.password.clone(), userdata.confirm_password.clone());

    let name: String;

    if error_username || error_email || error_password || error_confirm_password {
        name = format!("Registration faild - error");
        let context = TemplateContextRegister {name, error_username, error_email, error_password, error_confirm_password};
        return Err(Template::render("register", &context));
    }
    else {
        let password = userdata.password.clone();

        match hash(&password, DEFAULT_COST) {
            Ok(hashed_password) => {
                let connection = establish_connection();
                create_new_user(&connection,
                    userdata.username.clone(), 
                    hashed_password.clone(), 
                    userdata.email.clone()
                );
                // name = format!("username: {}\npassword: {}", userdata.username, hashed_password);
                return Ok(Redirect::to("/login"));
            }
            Err(_) => {
                name = format!("Registration faild");
            }
        }

        let context = TemplateContextRegister {name, error_username, error_email, error_password, error_confirm_password};
        return Err(Template::render("register", &context));
    }
}

#[get("/register")]
pub fn register_get() -> Template {
    let name = "".to_string();
    let context = TemplateContextRegister {
        name, 
        error_username: false, 
        error_email: false, 
        error_password: false,
        error_confirm_password: false,
    };
    Template::render("register", &context)
}

//<-----------------Home----------------->
fn get_user_id_from_session_token(session_token: String) -> Result<i64, std::io::Error> {
    use hello_rocket::schema::user_sessions::dsl::*;

    let connection = establish_connection();

    let results = user_sessions
        .filter(token.eq(session_token))
        .limit(1)
        .load::<UserSession>(&connection)
        .expect("Error loading sessions");
    
    if results.len() == 1 {
        return Ok(results[0].user_id as i64);
    }
    else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no token found",
        ));
    }
}

fn get_user_id_from_cookies(mut cookies: Cookies) -> Result<i64, std::io::Error> {
    match cookies.get_private("session_token") {
        Some(cookie) => match get_user_id_from_session_token(cookie.value().to_string()) {
            Ok(user_id) => Ok(user_id),
            Err(error) => Err(error),
        },
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no token found",
            ));
        }
    }
}

#[derive(Serialize)]
struct TemplateContextIndex {
    name: String,
    is_authenticated: bool,
}

fn check_user_id(user_id: i32) -> bool {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results = users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading email");

    if results.len() == 1 {
        return true;
    }
    else {
        return false;
    }

}


#[get("/")]
pub fn index(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                let context = TemplateContextIndex {
                    name: "TO DO - home - you are logged in".to_string(),
                    is_authenticated: true
                };
                return Template::render("index", &context);
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - home - you are not logged in".to_string(),
                    is_authenticated: false
                };
                return Template::render("index", &context);
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - home - login Page".to_string(),
                is_authenticated: false
            };
            return Template::render("index", &context);
        }

    }
}

//<-----------------Chart----------------->
#[get("/chart")]
pub fn chart(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                let context = TemplateContextIndex {
                    name: "".to_string(),
                    is_authenticated: true
                };
                return Template::render("chart", &context);
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - home - you are not logged in".to_string(),
                    is_authenticated: false
                };
                return Template::render("index", &context);
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - home - login Page".to_string(),
                is_authenticated: false
            };
            return Template::render("index", &context);
        }

    }
}

//<-----------------wallet----------------->
fn get_transactions_from_db(current_user_id: i32) -> Vec<Transaction> {
    use hello_rocket::schema::transactions::dsl::*;

    let connection = establish_connection();

    let results: Vec<Transaction> = transactions
        .filter(user_id.eq(current_user_id))
        .load::<Transaction>(&connection)
        .expect("Error loading sessions");
        
    results

    
} 

#[derive(FromForm)]
pub struct TransactionForm {
    date_transaction: String,
    sell_amount: f32,
    sell_currency: String,
    buy_amount: f32,
    buy_currency: String,
}

#[derive(Serialize)]
struct TemplateContextWallet {
    name: String,
    is_authenticated: bool,
    context_transactions: Vec<Transaction>,
}

#[get("/wallet")]
pub fn wallet_get(cookies: Cookies) -> Template {
    let mut context_transactions: Vec<Transaction>;

    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                context_transactions = get_transactions_from_db(user_id as i32);

                let context = TemplateContextWallet {
                    name: "".to_string(),
                    is_authenticated: true,
                    context_transactions,
                };
                return Template::render("wallet", &context);
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - wallet - you are not logged in".to_string(),
                    is_authenticated: false,
                };
                return Template::render("index", &context);
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - wallet - login Page".to_string(),
                is_authenticated: false,
            };
            return Template::render("index", &context);
        }

    }
}

#[post("/wallet/add_transaction", data = "<transaction>")]
pub fn wallet_post_add(mut cookies: Cookies, transaction: Form<TransactionForm>) -> Result<Redirect, Template> {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                let context = TemplateContextIndex {
                    name: "TO DO - wallet - you are logged in".to_string(),
                    is_authenticated: true
                };

                let mut price_for_one = 0.0;

                if transaction.sell_amount != 0.0 {
                    price_for_one = transaction.buy_amount/transaction.sell_amount;
                }

                let connection = establish_connection();
                create_new_transaction(&connection,
                    user_id as i32,
                    transaction.date_transaction.to_string().clone(),
                    transaction.sell_amount.clone(),
                    transaction.sell_currency.to_string().clone(),
                    transaction.buy_amount.clone(),
                    transaction.buy_currency.to_string().clone(),
                    price_for_one,
                );

                return Ok(Redirect::to("/wallet"));
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - wallet - you are not logged in".to_string(),
                    is_authenticated: false
                };
                return Err(Template::render("index", &context));
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - wallet - login Page".to_string(),
                is_authenticated: false
            };
            return Err(Template::render("index", &context))
        }

    }
}

fn remove_transaction(active_user_id: i32, transaction_id: String) {
    use hello_rocket::schema::transactions::dsl::*;

    let transaction_id: i32 = transaction_id.parse().unwrap();
    let connection = establish_connection();

    let num_deleted = diesel::
        delete(
            transactions
            .filter(user_id.eq(active_user_id))
            .filter(id.eq(transaction_id))
        )
        .execute(&connection)
        .expect("Error deleting posts");

}

#[post("/wallet/remove_transactions", data = "<data>")]
pub fn wallet_post_remove(content_type: &ContentType, data: Data, mut cookies: Cookies) -> Result<Redirect, Template> {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                // let context = TemplateContextIndex {
                //     name: "TO DO - wallet - you are logged in".to_string(),
                //     is_authenticated: true
                // };

                let mut options = MultipartFormDataOptions::new();
                options.allowed_fields
                       .push(MultipartFormDataField::text("transactions_to_remove").content_type(Some(mime::STAR_STAR))
                       .repetition(Repetition::infinite()));

                let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

                let transactions_to_remove = multipart_form_data.texts.get("transactions_to_remove");

                if let Some(transactions_to_remove) = transactions_to_remove {
                    match transactions_to_remove {
                        TextField::Single(transaction) => {
                            let _content_type = &transaction.content_type;
                            let _file_name = &transaction.file_name;
                            let transaction_id = &transaction.text;
                            // You can now deal with the text data.
                            remove_transaction(user_id as i32, transaction_id.to_string());
                        }
                        TextField::Multiple(transactions) => {
                            // Because we put "array_max_length_3" field to the allowed_fields for three times, this arm will probably be matched.
            
                            for transaction in transactions { // The max length of the "texts" variable is 3
                                let _content_type = &transaction.content_type;
                                let _file_name = &transaction.file_name;
                                let transaction_id = &transaction.text;
                                // You can now deal with the text data.
                                remove_transaction(user_id as i32, transaction_id.to_string());
                            }
                        }
                    }
                }

                return Ok(Redirect::to("/wallet"));
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - wallet - you are not logged in".to_string(),
                    is_authenticated: false
                };
                return Err(Template::render("index", &context));
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - wallet - login Page".to_string(),
                is_authenticated: false
            };
            return Err(Template::render("index", &context))
        }

    }
}

//<-----------------Logout----------------->
fn remove_user_id_from_session_token(session_token: String) {
    use hello_rocket::schema::user_sessions::dsl::*;

    let connection = establish_connection();

    let num_deleted = diesel::delete(user_sessions.filter(token.eq(session_token)))
        .execute(&connection)
        .expect("Error deleting posts");

}

// Remove the `user_id` cookie.
#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    
    if let Some(cookie) = cookies.get_private("session_token") {
        remove_user_id_from_session_token(cookie.value().to_string());
        cookies.remove_private(Cookie::named("session_token"));
    }
    
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

