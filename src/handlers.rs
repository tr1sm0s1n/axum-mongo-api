use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::Collection;

use crate::models;

pub async fn count(State(db): State<Collection<models::Certificate>>) -> impl IntoResponse {
    let count = db.count_documents(None, None).await.unwrap();

    (StatusCode::OK, Json(count))
}
