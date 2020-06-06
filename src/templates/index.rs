use hello_rocket::*;
use hello_rocket::models::{UserSession, User};
use diesel::prelude::*;
use rocket::http::Cookies;

#[derive(Serialize)]
pub struct TemplateContextIndex {
    pub name: String,
    pub is_authenticated: bool,
}

pub fn get_user_id_from_session_token(session_token: String) -> Result<i64, std::io::Error> {
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

pub fn get_user_id_from_cookies(cookies: &mut Cookies) -> Result<i64, std::io::Error> {
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

pub fn check_user_id(user_id: i32) -> bool {
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