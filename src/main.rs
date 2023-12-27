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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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



