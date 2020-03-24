use rocket_contrib::templates::Template;
use rocket::request::Form;
use hello_rocket::establish_connection;
use hello_rocket::create_new_user;



#[derive(Serialize)]
struct TemplateContext {
    name: String,
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

#[derive(FromForm)]
pub struct User {
    username: String,
    password: String,
}

#[post("/register", data = "<userdata>")]
pub fn register_post(userdata: Form<User>) -> Template {
    let connection = establish_connection();
    create_new_user(&connection, userdata.username.clone(), userdata.password.clone());
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
