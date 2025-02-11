use rocket::routes;

mod user_create;
mod check_admin_token;
mod admin_login;


pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        admin_login::admin_login,
        check_admin_token::check_admin_token,
    ]
}
