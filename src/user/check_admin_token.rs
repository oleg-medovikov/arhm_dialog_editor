use rocket::{post, serde::json::Json, State, response::Debug};
use sqlx::{PgPool, query_scalar};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenRequest {
    token: String,
}

#[derive(Serialize)]
pub struct TokenStatus {
    token_valid: bool,
}

#[post("/check_admin_token", format = "json", data = "<token_request>")]
pub async fn check_admin_token(pool: &State<PgPool>, token_request: Json<TokenRequest>) -> Result<Json<TokenStatus>, Debug<sqlx::Error>> {
    // Ищем пользователя по токену
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE token = $1 and admin = true)"#)
        .bind(&token_request.token)
        .fetch_one(pool.inner())
        .await
        .map_err(Debug)?;

    let token_status = TokenStatus { token_valid: exists };
    Ok(Json(token_status))
}
