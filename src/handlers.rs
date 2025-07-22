use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::models::Certificate;

pub async fn count(
    State(db): State<Collection<Certificate>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = db.count_documents(doc! {}).await.map_err(internal_error)?;

    Ok(Json(count))
}

pub async fn create(
    State(db): State<Collection<Certificate>>,
    Json(input): Json<Certificate>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = db.insert_one(input).await.map_err(internal_error)?;
    println!("Inserted a document with _id: {}", result.inserted_id);

    Ok(Json(result))
}

pub async fn read_all(
    State(db): State<Collection<Certificate>>,
) -> Result<Json<Vec<Certificate>>, (StatusCode, String)> {
    let mut cursor = db.find(doc! {}).await.unwrap();

    let mut certificates = Vec::new();
    while let Some(c) = cursor.try_next().await.map_err(internal_error)? {
        println!("{:?}", c);
        certificates.push(c)
    }

    Ok(Json(certificates))
}

pub async fn read_one(
    Path(id): Path<u32>,
    State(db): State<Collection<Certificate>>,
) -> Result<Json<Option<Certificate>>, (StatusCode, String)> {
    let result = db
        .find_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;
    println!("{:?}", result);

    Ok(Json(result))
}

pub async fn update_one(
    Path(id): Path<u32>,
    State(db): State<Collection<Certificate>>,
    Json(input): Json<Certificate>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = db
        .replace_one(doc! { "_id": id }, input)
        .await
        .map_err(internal_error)?;
    println!("Updated {:?} document(s)", result.modified_count);

    Ok(Json(result))
}

pub async fn delete_one(
    Path(id): Path<u32>,
    State(db): State<Collection<Certificate>>,
) -> Result<Json<DeleteResult>, (StatusCode, String)> {
    let result = db.delete_one(doc! { "_id": id }).await.unwrap();
    println!("Deleted {:?} document(s)", result.deleted_count);

    Ok(Json(result))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
