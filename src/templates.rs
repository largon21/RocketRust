use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies}; 
use rocket::response::Redirect; 
use rocket::request::Form;
use hello_rocket::*;
use time::Duration;




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
        // "Username or password incorrect"
    }
    
    Err(Template::render("login", &context))
}

//<-----------------Home----------------->
#[get("/")]
pub fn index() -> Template {
    let name = "TO DO - home".to_string();
    let context = TemplateContext {name};
    Template::render("index", &context)
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
    let connection = establish_connection();
    create_new_user(&connection,
        userdata.username.clone(), 
        userdata.password.clone(), 
        userdata.email.clone()
    );

    let name = format!("username: {}\npassword: {}", userdata.username, userdata.password);
    let context = TemplateContext {name};
    Template::render("register", &context)
}

#[get("/register")]
pub fn register_get() -> Template {
    let name = "TO DO - register".to_string();
    let context = TemplateContext {name};
    Template::render("register", &context)
}


