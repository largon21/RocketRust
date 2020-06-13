use hello_rocket::*;
use hello_rocket::models::User;
use diesel::prelude::*;
use bcrypt::verify;

#[derive(Serialize)]
pub struct TemplateContextAccount {
    pub username_account: String,
    pub email_account: String,
    pub tab_selected: String,

    pub error_username: bool,
    pub error_username_password: bool,
    pub error_email: bool,
    pub error_email_password: bool,
    pub error_current_password: bool,
    pub error_password: bool,
    pub error_confirm_password: bool,
    pub error_remove_account_password: bool,
    pub error_remove_account_confirm_password: bool,
    pub is_authenticated: bool,
}

#[derive(FromForm)]
pub struct UserFormAccountPassword {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(FromForm)]
pub struct UserFormAccountUsername {
    pub current_password: String,
    pub new_username: String,
}

#[derive(FromForm)]
pub struct UserFormAccountEmail {
    pub current_password: String,
    pub new_email: String,
}

#[derive(FromForm)]
pub struct UserFormRemoveAccount {
    pub current_password: String,
    pub confirm_password: String,
}


pub fn remove_account(username: String, active_user_id: i32){
    use hello_rocket::schema::transactions::dsl::*;
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let _deleted_transactions = diesel::
        delete(transactions.filter(user_id.eq(active_user_id)))
        .execute(&connection)
        .expect("Error deleting transactions account");

    let _deleted_user = diesel::delete(users.filter(nickname.eq(username)))
        .execute(&connection)
        .expect("Error deleting user");

}

pub fn error_account_validate_password(user_id: i32, current_password: String) -> bool {
    match get_password_hash_from_id(user_id.clone()) {
        Ok(password_hash) => {
            match verify(&current_password, &password_hash) {
                Ok(password_match) => {
                    if password_match {
                        return false;
                    }
                    else {
                        return true;
                    }
                }
                Err(_) => {
                    return true;
                }
            }
        }
        Err(_) => {
            return true;
        }
    }

}

pub fn get_password_hash_from_id(user_id: i32) -> Result<String, std::io::Error> {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results = users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading user");
    
    
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

pub fn get_nickname_from_id(user_id: i32) -> String {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results = users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading user");
    
    
    
    return results[0].nickname.to_string();
}

pub fn get_email_from_id(user_id: i32) -> String {
    use hello_rocket::schema::users::dsl::*;

    let connection = establish_connection();

    let results = users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error loading user");
    
    
    
    return results[0].email.to_string();
}

pub fn update_account_nickname(nickname: String, new_nickname: String) {
    let connection = establish_connection();

    update_user_nickname(&connection, nickname, new_nickname);
}

pub fn update_account_email(nickname: String, new_email: String) {
    let connection = establish_connection();
    
    update_user_email(&connection, nickname, new_email);
}

pub fn update_account_password(nickname: String, new_password: String) {
    let connection = establish_connection();
    
    update_user_password(&connection, nickname, new_password);
}