use crate::{
    model::{BookModel, BookModelResponse},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use std::sync::Arc;

fn to_book_response(book: &BookModel) -> BookModelResponse {
    BookModelResponse {
        id: book.id.to_owned(),
        title: book.title.to_owned(),
        author: book.author.to_owned(),
        page: book.page.to_owned(),
        note: book.note.to_owned(),
    }
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn book_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let books = sqlx::query!("SELECT * FROM book")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let mut book_response: Vec<BookModelResponse> = Vec::new();

    for book in books {
        let row: BookModel = BookModel {
            id: book.id,
            title: book.title.unwrap(),
            author: book.author.unwrap(),
            page: book.page.unwrap(),
            note: book.note.unwrap(),
        };

        book_response.push(to_book_response(&row));
    }

    let json_response = serde_json::json!({
        "status": "OK",
        "count": book_response.len(),
        "books": book_response
    });

    Ok(Json(json_response))
}
