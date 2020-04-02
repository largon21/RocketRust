use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies}; 
use rocket::response::Redirect; 
use rocket::request::Form;
use hello_rocket::*;
use time::Duration;
use hello_rocket::models::UserSession;
use diesel::prelude::*;
use bcrypt::{DEFAULT_COST, hash, verify};



#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[derive(FromForm)]
pub struct User {
    username: String,
    password: String,
    email: String,
}

//<-----------------Login----------------->
fn generate_session_token(length: u8) -> Result<String, std::io::Error> {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    let strings: Vec<String> = bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    return Ok(strings.join(""));
}

#[post("/login", data = "<user>")]
pub fn login(mut cookies: Cookies, user: Form<User>) -> Result<Redirect, Template> {
    let name = "TO DO - login".to_string();
    let context = TemplateContext {name};

    if user.username == "true".to_string() && user.password == "true".to_string() {
        match generate_session_token(64) {
            Ok(session_token) => {
                let connection = establish_connection();
                let user_id = 1;
                create_new_user_session(&connection, user_id, session_token.clone());
                let mut c = Cookie::new("session_token", session_token);
                c.set_max_age(Duration::hours(24));
                cookies.add_private(c);
                return Ok(Redirect::to("/"));
            }
            Err(_) => {
                return Err(Template::render("login", &context));
            }    
        }
    } 
    else {
        let context = TemplateContext {
            name: "Username or password incorrect".to_string()
        };
    }
    
    Err(Template::render("login", &context))
}

//<-----------------Home----------------->
fn get_user_id_from_Session_token(session_token: String) -> Result<i64, std::io::Error> {
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
        Some(cookie) => match get_user_id_from_Session_token(cookie.value().to_string()) {
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

#[get("/")]
pub fn index(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if user_id == 1 {
                let context = TemplateContext {
                    name: "TO DO - home - you are logged in".to_string()
                };
                return Template::render("index", &context);
            } 
            else {
                let context = TemplateContext {
                    name: "TO DO - home - you are not logged in".to_string()
                };
                return Template::render("index", &context);
            }
        }
        Err(_not_logged_in) => {
            let context = TemplateContext {
                name: "TO DO - home - login Page".to_string()
            };
            return Template::render("index", &context);
        }

    }
}

//<-----------------About----------------->
#[get("/about")]
pub fn about() -> Template {
    let name = "TO DO - about".to_string();
    let context = TemplateContext {name};
    Template::render("about", &context)
}


//<-----------------Register----------------->

#[post("/register", data = "<userdata>")]
pub fn register_post(userdata: Form<User>) -> Template {
    
    let password = userdata.password.clone();
    let name: String;

    match hash(&password, DEFAULT_COST) {
        Ok(hashed_password) => {
            let connection = establish_connection();
            create_new_user(&connection,
                userdata.username.clone(), 
                hashed_password.clone(), 
                userdata.email.clone()
            );
            name = format!("username: {}\npassword: {}", userdata.username, hashed_password);
            
        }
        Err(_) => {
            name = format!("registration faild");
        }
    }

    let context = TemplateContext {name};
    Template::render("register", &context)
}

#[get("/register")]
pub fn register_get() -> Template {
    let name = "TO DO - register".to_string();
    let context = TemplateContext {name};
    Template::render("register", &context)
}


