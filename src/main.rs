#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

// extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod templates;


fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))) //css file
        .mount("/", routes![
            templates::index, 
            templates::account, 
            templates::change_password_post,
            templates::change_username_post,
            templates::change_email_post,
            templates::remove_account_post,
            templates::chart, 
            templates::wallet_transactions_get,
            templates::wallet_transactions_post_add,
            templates::wallet_transactions_post_remove,
            templates::wallet_dashboard_get,
            templates::register_get, 
            templates::register_post,
            templates::login_get,
            templates::login_post,
            templates::logout,
            ])
        .attach(Template::fairing())
        .launch();
}