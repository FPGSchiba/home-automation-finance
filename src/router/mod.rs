use std::error::Error;

use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use expenses::get_expense_router;
use groups::get_group_router;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::USER_API_URL;

mod expenses;
mod groups;

#[derive(Serialize)]
struct Version {
    version: String,
}

#[derive(Serialize, Deserialize)]
struct Route {
    path: String,
    methods: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Permission {
    routes: Vec<Route>,
}

#[derive(Serialize, Deserialize)]
struct PermissionsResponse {
    message: String,
    code: String,
    permissions: Vec<Permission>,
}

async fn get_permissions_from_token(token: &str) -> Result<Vec<Permission>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(USER_API_URL)
        .header(AUTHORIZATION, token)
        .send()
        .await?;

    let value: PermissionsResponse = res.json().await?;
    Ok(value.permissions)
}

async fn auth(request: Request, next: Next) -> Response {
    let auth_header = match request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        Some(token) => token,
        None => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::empty())
                .unwrap();
        }
    };

    match get_permissions_from_token(auth_header).await {
        Ok(_permissions) => next.run(request).await,
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::empty())
            .unwrap(),
    }
}

pub fn get_router() -> Router {
    let group_router = get_group_router();
    let expense_router = get_expense_router();

    Router::new()
        .route("/api/v1/", get(version))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .route_layer(middleware::from_fn(auth))
        .nest("/groups", group_router)
        .nest("/expenses", expense_router)
}

async fn version() -> Json<Version> {
    Json(Version {
        version: "0.1.0".to_owned(),
    })
}
