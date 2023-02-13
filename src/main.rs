mod entity;
mod service;
mod app;

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

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8090));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    use crate::app::create_app;

    #[tokio::test]
    async fn test_hello_router() {
        let router = create_app().await;
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "{\"message\":\"API Rust with Axum is working!!!\"}");
    }

    #[tokio::test]
    async fn test_health_router() {
        let router = create_app().await;
        let client = TestClient::new(router);
        let res = client.get("/health").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "{\"status\":\"UP\"}");
    }

    #[tokio::test]
    async fn test_get_all_books_router() {
        let router = create_app().await;
        let client = TestClient::new(router);
        let res = client.get("/books").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "[]");
    }
}



