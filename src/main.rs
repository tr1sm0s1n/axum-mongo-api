mod config;
mod handlers;
mod models;

use axum::{routing::get, Router};
use mongodb::{Client, Collection};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // connecting to mongodb
    let client = config::connect().await.unwrap();

    // logging middleware
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_mongo_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(client)).await.unwrap();
}

fn app(client: Client) -> Router {
    let collection: Collection<models::Certificate> =
        client.database("axum-mongo").collection("certificates");

    Router::new()
        .route("/", get(home))
        .route("/count", get(handlers::count))
        .layer(TraceLayer::new_for_http())
        .with_state(collection)
}

async fn home() -> &'static str {
    "Hello, World!"
}
