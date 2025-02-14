use rocket::routes;

mod get_dialogs;


pub fn dialog_routes() -> Vec<rocket::Route> {
    routes![
        get_dialogs::get_dialogs,
    ]
}


