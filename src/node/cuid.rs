use rocket::{get, serde::json::Json};
use serde::Serialize;
use cuid::cuid2;

// Структура для ответа JSON
#[derive(Serialize)]
pub struct CuidResponse {
    ids: Vec<String>,
}

// Эндпоинт для генерации нескольких CUID
#[get("/cuid/<count>")]
pub fn generate_cuids(count: usize) -> Json<CuidResponse> {
    // Генерация указанного количества уникальных CUID
    let ids: Vec<String> = (0..count).map(|_| cuid2()).collect();

    // Создание структуры ответа
    let response = CuidResponse { ids };

    // Возвращаем JSON-ответ
    Json(response)
}
