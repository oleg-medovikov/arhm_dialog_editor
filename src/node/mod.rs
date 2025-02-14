use rocket::routes;

mod cuid;


pub fn node_routes() -> Vec<rocket::Route> {
    routes![
        cuid::generate_cuids,
    ]
}
