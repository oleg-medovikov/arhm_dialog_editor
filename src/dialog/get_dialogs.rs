use rocket::{get, serde::json::Json, State, response::status::Custom, http::Status};
use sqlx::PgPool;
use crate::models::Event;
use crate::system::admin_token::AdminToken;

#[get("/dialogs")]
pub async fn get_dialogs(pool: &State<PgPool>, _user_guid: AdminToken) -> Result<Json<Vec<Event>>, Custom<String>> {
    let query = r#"
        SELECT 
            d.id, d.name, d.code, d.description, d.image,
           to_char(e.date_create AT TIME ZONE 'Europe/Moscow', 'YYYY-MM-DD HH24:MI:SS') as date_create,
           to_char(e.date_update AT TIME ZONE 'Europe/Moscow', 'YYYY-MM-DD HH24:MI:SS') as date_update,
           u.username as user
        FROM dialog d
        inner join users u on e.user_id = u.id
        ORDER BY date_update DESC;"#;

    let events: Vec<Event> = match sqlx::query_as::<_, Event>(query)
        .fetch_all(pool.inner())
        .await
    {
        Ok(events) => events,
        Err(e) => {
            return Err(Custom(Status::InternalServerError, format!("Database error: {}", e),));
        }
    };

    Ok(Json(events))
}
