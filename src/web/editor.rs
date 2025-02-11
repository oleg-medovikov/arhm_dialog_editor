use rocket::response::content::{RawHtml, RawCss, RawJavaScript};
use std::fs;
use rocket::get;
//use rocket::uri;
//use rocket::response::Redirect;

//use crate::system::admin_token::{AdminToken, ApiTokenError};


#[get("/")]
pub fn editor_main() -> RawHtml<String> {
    let content = fs::read_to_string("static/index.html").expect("Unable to read file");
    RawHtml(content)
}

#[get("/editor_main.css")]
pub fn editor_main_css() -> RawCss<String> {
    let content = fs::read_to_string("static/css/main.css").expect("Unable to read file");
    RawCss(content)
}

#[get("/loginCheck.js")]
pub fn login_check_js() -> RawJavaScript<String> {
    let content = fs::read_to_string("static/js/loginCheck.js").expect("Unable to read file");
    RawJavaScript(content)
}

#[get("/go.js")]
pub fn go_js() -> RawJavaScript<String> {
    let content = fs::read_to_string("static/js/go.js").expect("Unable to read file");
    RawJavaScript(content)
}

#[get("/code.js")]
pub fn code_js() -> RawJavaScript<String> {
    let content = fs::read_to_string("static/js/code.js").expect("Unable to read file");
    RawJavaScript(content)
}

#[get("/contentLoad.js")]
pub fn content_load_js() -> RawJavaScript<String> {
    let content = fs::read_to_string("static/js/contentLoad.js").expect("Unable to read file");
    RawJavaScript(content)
}
