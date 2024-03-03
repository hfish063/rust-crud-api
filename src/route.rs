use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handler::{book_list_handler, health_check_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/books", get(health_check_handler))
        .route("/api/books/all", get(book_list_handler))
        .with_state(app_state)
}
