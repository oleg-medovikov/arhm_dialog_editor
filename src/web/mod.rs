use rocket::routes;

mod editor;
mod login;

pub fn web_routes() -> Vec<rocket::Route> {
    routes![
        login::login,
        login::login_js,
        login::login_css,
        editor::editor_main,
        editor::editor_main_css,
        editor::login_check_js,
        editor::go_js,
        editor::code_js,
        editor::content_load_js,
    ]
}
