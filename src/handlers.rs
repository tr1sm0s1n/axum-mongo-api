use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::Collection;

use crate::models::Certificate;

pub async fn count(State(db): State<Collection<Certificate>>) -> impl IntoResponse {
    let count = db.count_documents(None, None).await.unwrap();

    (StatusCode::OK, Json(count))
}

pub async fn create(
    State(db): State<Collection<Certificate>>,
    Json(input): Json<Certificate>,
) -> impl IntoResponse {
    let result: mongodb::results::InsertOneResult = db.insert_one(input, None).await.unwrap();
    println!("Inserted a document with _id: {}", result.inserted_id);

    (StatusCode::CREATED, Json(result))
}
