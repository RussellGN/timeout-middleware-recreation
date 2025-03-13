use std::{collections::HashMap, time::Duration};

use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(sleep_handler));

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
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
    let duration = Duration::from_secs(timeout_length + sleep_length);

    tokio::time::sleep(duration).await;

    format!("Slept for: {sleep_length}, Timeout set: {timeout_length}",)
}
