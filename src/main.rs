use axum::{Json, Router, response::Html, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/info", get(info));
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.expect("bind");
    println!("cnp-axum-demo écoute sur 0.0.0.0:{port}");
    axum::serve(listener, app).await.expect("serveur HTTP");
}

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn info() -> Json<Value> {
    Json(json!({
        "service": "cnp-axum-demo",
        "language": "rust",
        "framework": "axum",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
