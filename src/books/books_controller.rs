use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn get_router() -> Router {
    Router::new().route("/getBooks", get(get_books))
}

async fn get_books() -> Json<Value> {
    Json(json!({"getlibros": "libro"}))
}
