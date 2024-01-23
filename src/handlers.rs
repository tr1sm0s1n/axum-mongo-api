use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};

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

pub async fn read_all(State(db): State<Collection<Certificate>>) -> impl IntoResponse {
    let mut cursor = db.find(None, None).await.unwrap();

    let mut certificates = Vec::new();
    while let Some(c) = cursor.try_next().await.unwrap() {
        println!("{:?}", c);
        certificates.push(c)
    }

    (StatusCode::OK, Json(certificates))
}

pub async fn read_one(
    Path(id): Path<u32>,
    State(db): State<Collection<Certificate>>,
) -> impl IntoResponse {
    let result = db.find_one(doc! { "_id": id }, None).await.unwrap();
    println!("{:?}", result);
    if result.is_none() {
        return (StatusCode::NOT_FOUND, Json(result));
    }

    (StatusCode::OK, Json(result))
}
