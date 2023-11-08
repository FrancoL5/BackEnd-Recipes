use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let addr = &"0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    info!("vistó la ruta '/'");
    Json(json!({"prueba":123}))
}
