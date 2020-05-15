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
use hello_rocket::models::Transaction;
use bcrypt::{DEFAULT_COST, hash, verify};

pub mod login;
pub use login::{
    UserLoginForm, TemplateContextLogin, 
    get_password_hash_from_username_or_email,
    get_user_id_from_username_or_email,
    generate_session_token,
    login_validate_form
    };

pub mod register;
pub use register::{
    UserForm, TemplateContextRegister, 
    register_validate_username,
    register_validate_email,
    register_validate_password,
    register_validate_confirm_password
    };

pub mod index;
pub use index::{
    TemplateContextIndex, 
    get_user_id_from_session_token,
    get_user_id_from_cookies,
    check_user_id
    };

pub mod wallet;
pub use wallet::{
    TransactionForm, 
    TemplateContextWallet,
    remove_transaction,
    get_transactions_from_db
    };
    
pub mod logout;
pub use logout::remove_user_id_from_session_token;

//<--------------------Account---------------------->
#[get("/account")]
pub fn account(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
                let context = TemplateContextIndex {
                    name: "TO DO - account - you are logged in".to_string(),
                    is_authenticated: true
                };
                return Template::render("account", &context);
            } 
            else {
                let context = TemplateContextIndex {
                    name: "TO DO - account - you are not logged in".to_string(),
                    is_authenticated: false
                };
                return Template::render("account", &context);
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContextIndex {
                name: "TO DO - account - login Page".to_string(),
                is_authenticated: false
            };
            return Template::render("account", &context);
        }

    }
}

//<-----------------Login----------------->
#[get("/login")]
pub fn login_get() -> Template {
    let context = TemplateContextLogin {
        name: "".to_string(), 
        error_username: false, 
        error_password: false
    };
    Template::render("login", &context)
}

#[post("/login", data = "<user>")]
pub fn login_post(mut cookies: Cookies, user: Form<UserLoginForm>) -> Result<Redirect, Template> {
    let error_username = login_validate_form(user.username.clone());
    let error_password = login_validate_form(user.password.clone());


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

//<-----------------Register----------------->
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

//<-----------------Index----------------->
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
#[get("/wallet")]
pub fn wallet_get(cookies: Cookies) -> Template {
    let context_transactions: Vec<Transaction>;

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
pub fn wallet_post_add(cookies: Cookies, transaction: Form<TransactionForm>) -> Result<Redirect, Template> {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {
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

#[post("/wallet/remove_transactions", data = "<data>")]
pub fn wallet_post_remove(cookies: Cookies, content_type: &ContentType, data: Data) -> Result<Redirect, Template> {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if check_user_id(user_id as i32) {

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
                            // can now deal with the text data.
                            remove_transaction(user_id as i32, transaction_id.to_string());
                        }
                        TextField::Multiple(transactions) => {
            
                            for transaction in transactions {
                                let _content_type = &transaction.content_type;
                                let _file_name = &transaction.file_name;
                                let transaction_id = &transaction.text;
                                // can now deal with the text data.
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
#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    // Remove the `user_id` cookie.
    if let Some(cookie) = cookies.get_private("session_token") {
        remove_user_id_from_session_token(cookie.value().to_string());
        cookies.remove_private(Cookie::named("session_token"));
    }
    
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

