use std::env;
use sqlx::PgPool;
use dotenv::dotenv;

mod user;
mod models;
mod system;
mod create_tables;
mod web;
mod node;


#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL не задан в конфиге");
    let pool = PgPool::connect(&database_url).await.expect("No DB CONNECT!");

    create_tables::create_tables(&pool).await.expect("Не удалось создать таблицы");

    rocket::build()
        .mount("/", web::web_routes())
        .mount("/api", user::user_routes())
        .mount("/api", node::node_routes())
        .manage(pool)
}
