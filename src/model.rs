use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub page: i32,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BookModelResponse {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub page: i32,
    pub note: String,
}
