mod books;
mod structs;

use axum::{routing::get, Json, Router};
use books::books_controller;
use dotenv::dotenv;
use reqwest::StatusCode;
use serde_json::{json, Value};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let api_router = Router::new()
        .route("/", get(root))
        .nest("/books", books_controller::get_router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = &"0.0.0.0:3000".parse().unwrap();

    let app = Router::new().nest("/api", api_router).fallback(fallback);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    info!("vistó la ruta '/'");
    Json(json!({"prueba":123,}
    ))
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}
