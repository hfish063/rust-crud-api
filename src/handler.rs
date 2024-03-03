use crate::{model::BookModel, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use std::sync::Arc;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Json(json_response)
}

/**
 * Returns list of all records stored in book table
 * State(data) holds the shared program state from main.rs, containing copy of database pool
 */
pub async fn book_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let books = sqlx::query!("SELECT * FROM book")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let mut book_response: Vec<BookModel> = Vec::new();

    for book in books {
        // SqlRow fields should obtained from db should not be empty
        let row: BookModel = BookModel {
            id: book.id,
            title: book.title.unwrap(),
            author: book.author.unwrap(),
            page: book.page.unwrap(),
            note: book.note.unwrap(),
        };

        book_response.push(row);
    }

    let json_response = serde_json::json!({
        "status": "OK",
        "count": book_response.len(),
        "books": book_response,
    });

    Ok(Json(json_response))
}

/**
 * Returns single record of book table with corresponding id field
 * State(data) holds the shared program state from main.rs, containing copy of database pool
 */
pub async fn book_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(serde_json::json!({"status": "OK"})))
}
