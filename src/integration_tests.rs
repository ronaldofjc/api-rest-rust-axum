
#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;
    use crate::app::create_app;
    use crate::entity::book::Book;

    #[tokio::test]
    async fn test_hello_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.get("/").await;
        assert_eq!(res.status_code(), StatusCode::OK);
        assert_eq!(res.text(), "{\"message\":\"API Rust with Axum is working!!!\"}");
    }

    #[tokio::test]
    async fn test_health_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.get("/health").await;
        assert_eq!(res.status_code(), StatusCode::OK);
        assert_eq!(res.text(), "{\"status\":\"UP\"}");
    }

    #[tokio::test]
    async fn test_ping_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.get("/ping").await;
        assert_eq!(res.status_code(), StatusCode::OK);
        assert_eq!(res.text(), "{\"message\":\"pong\"}");
    }

    #[tokio::test]
    async fn test_get_all_books_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.get("/books").await;
        assert_eq!(res.status_code(), StatusCode::OK);
        assert_eq!(res.text(), "[]");
    }

    #[tokio::test]
    async fn test_get_create_new_book_with_success_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien",
                "pages": 2000,
            })).await;
        assert_eq!(res.status_code(), StatusCode::CREATED);
        assert!(res.text().contains("\"title\":\"Lord of the Rings\""));
    }

    #[tokio::test]
    async fn test_get_create_new_book_with_invalid_params_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let res = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien"
            })).await;
        assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
        assert!(res.text().contains("\"message\":\"Invalid params on request\""));
    }

    #[tokio::test]
    async fn test_get_book_by_id_with_success_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let _ = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        let books = server.get("/books").await.json::<Vec<Book>>();
        let id = books[0].id;
        let url = "/books/".to_string() + &*id.to_string();
        let res = server.get(&*url).await;

        assert_eq!(res.status_code(), StatusCode::OK);
        assert!(res.text().contains("\"title\":\"Lord of the Rings\""));
    }

    #[tokio::test]
    async fn test_not_found_on_get_book_by_id_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let url = "/books/b3676354-e02e-4f2d-b1dc-ff3162a74e5b".to_string();
        let res = server.get(&*url).await;

        assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
        assert!(res.text().contains("\"message\":\"book not found\""));
    }

    #[tokio::test]
    async fn test_update_book_with_invalid_params_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let url = "/books/b3676354-e02e-4f2d-b1dc-ff3162a74e5b".to_string();
        let res = server.put(&*url)
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien"
            })).await;
        assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
        assert!(res.text().contains("\"message\":\"Invalid params on request\""));
    }

    #[tokio::test]
    async fn test_update_book_with_success_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let _ = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        let books = server.get("/books").await.json::<Vec<Book>>();
        let id = books[0].id;
        let url = "/books/".to_string() + &*id.to_string();

        let res = server.put(&*url)
            .json(&json!({
                "title": "Lord of the Rings 2",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        assert_eq!(res.status_code(), StatusCode::OK);
        assert!(res.text().contains("\"title\":\"Lord of the Rings 2\""));
    }

    #[tokio::test]
    async fn test_update_book_when_invalid_params_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let _ = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        let books = server.get("/books").await.json::<Vec<Book>>();
        let id = books[0].id;
        let url = "/books/".to_string() + &*id.to_string();

        let res = server.put(&*url)
            .json(&json!({
                "title": "Lord of the Rings 2",
                "author": "Tolkien"
            })).await;

        assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
        assert!(res.text().contains("\"message\":\"Invalid params on request\""));
    }

    #[tokio::test]
    async fn test_update_book_when_not_found_router() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();
        let _ = server.post("/books")
            .json(&json!({
                "title": "Lord of the Rings",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        //let books = server.get("/books").await.json::<Vec<Book>>();
        let url = "/books/b3676354-e02e-4f2d-b1dc-ff3162a74e5b".to_string();

        let res = server.put(&*url)
            .json(&json!({
                "title": "Lord of the Rings 2",
                "author": "Tolkien",
                "pages": 2000,
            })).await;

        assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
        assert!(res.text().contains("\"message\":\"book not found\""));
    }
}