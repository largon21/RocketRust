use hello_rocket::*;
use hello_rocket::models::User;
use diesel::prelude::*;

#[derive(FromForm)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize)]
pub struct TemplateContextRegister {
    pub name: String,
    pub error_username: bool,
    pub error_email: bool,
    pub error_password: bool,
    pub error_confirm_password: bool,
}

pub fn error_register_validate_username(username: String) -> bool {
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

pub fn error_register_validate_email(user_email: String) -> bool {
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

pub fn error_register_validate_password(user_password: String) -> bool {
    if !user_password.is_empty() && user_password.len() >= 4 {
        return false;
    }    
    else {
        return true;
    }
}

pub fn error_register_validate_confirm_password(str1: String, str2: String) -> bool {
    if str1 == str2 {
        return false;
    }    
    else {
        return true;
    }
}