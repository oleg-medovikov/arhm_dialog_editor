use rocket::response::content::{RawHtml, RawCss, RawJavaScript};
use std::fs;
use rocket::get;


#[get("/login")]
pub fn login() -> RawHtml<String> {
    let content = fs::read_to_string("static/login.html").expect("Unable to read file");
    RawHtml(content)
}

#[get("/login.js")]
pub fn login_js() -> RawJavaScript<String> {
    let content = fs::read_to_string("static/js/login.js").expect("Unable to read file");
    RawJavaScript(content)
}

#[get("/login.css")]
pub fn login_css() -> RawCss<String> {
    let content = fs::read_to_string("static/css/login.css").expect("Unable to read file");
    RawCss(content)
}



