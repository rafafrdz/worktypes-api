#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use companies_api::{
        models::CompanyRequest, repository::CompanyRepository, routes::create_routes,
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    async fn setup() -> Router {
        let repository = Arc::new(CompanyRepository::new());
        create_routes(repository)
    }

    #[tokio::test]
    async fn test_list_companies() {
        let app = setup().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/companies")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_company() {
        let app = setup().await;

        let company_request = CompanyRequest {
            name: "Test Company".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/companies")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&company_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    // Más tests aquí...
}
