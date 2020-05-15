use hello_rocket::*;
use diesel::prelude::*;

pub fn remove_user_id_from_session_token(session_token: String) {
    use hello_rocket::schema::user_sessions::dsl::*;

    let connection = establish_connection();

    let _num_deleted = diesel::delete(user_sessions.filter(token.eq(session_token)))
        .execute(&connection)
        .expect("Error deleting posts");
}