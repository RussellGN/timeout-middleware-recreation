use std::{collections::HashMap, time::Duration};

use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(sleep_handler))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!(
        "server running on port {}",
        listener
            .local_addr()
            .expect("should be able to get local address")
            .to_string()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn sleep_handler(Query(params): Query<HashMap<String, u64>>) -> impl IntoResponse {
    let timeout_length = params.get("timeout").expect("timeout parameter").to_owned();
    let sleep_length = params.get("sleep").expect("sleep parameter").to_owned();
    let duration = Duration::from_secs(sleep_length);

    tokio::time::sleep(duration).await;

    format!("Slept for: {sleep_length}, Timeout set: {timeout_length}",)
}
