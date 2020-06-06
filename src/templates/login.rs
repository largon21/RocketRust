use hello_rocket::*;
use hello_rocket::models::User;
use diesel::prelude::*;

#[derive(FromForm)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TemplateContextLogin {
    pub name: String,
    pub error_username: bool,
    pub error_password: bool,
}

pub fn get_password_hash_from_username_or_email(name: String) -> Result<String, std::io::Error> {
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

pub fn get_user_id_from_username_or_email(name: String) -> Result<i32, std::io::Error> {
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

pub fn generate_session_token(length: u8) -> Result<String, std::io::Error> {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    let strings: Vec<String> = bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    return Ok(strings.join(""));
}

pub fn error_login_validate_empty_form(word: String) -> bool { //if error set true
    if word.is_empty() {
        return true;
    }
    else {
        return false;
    }
}