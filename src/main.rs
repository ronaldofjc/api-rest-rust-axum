mod entity;
mod service;
mod app;
mod integration_tests;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "axum_api=debug".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}