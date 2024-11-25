use std::{error::Error, str};

use axum::{
    body::Body,
    extract::Request,
    http::{self, HeaderMap, StatusCode},
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

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

async fn get_permissions_from_token(token: &String) -> Result<Vec<Permission>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(USER_API_URL)
        .header(AUTHORIZATION, token)
        .send()
        .await?;

    let value: PermissionsResponse = res.json().await?;
    Ok(value.permissions)
}

async fn auth(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token) if token_is_valid(&token).await => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_token(headers: &HeaderMap) -> Option<String> {
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        let auth_str = auth_header.to_str().unwrap();
        Some(auth_str.replace("Bearer ", "").to_string())
    } else {
        None
    }
}

async fn token_is_valid(token: &String) -> bool {
    if let Ok(permissions) = get_permissions_from_token(token).await {
        true
    } else {
        false
    }
}

pub fn get_router() -> Router {
    let group_router = get_group_router();
    let expense_router = get_expense_router();
    let api_router = Router::new()
        .nest("/groups", group_router)
        .nest("/expenses", expense_router)
        .route_layer(middleware::from_fn(auth));

    Router::new()
        .route("/", get(version))
        .nest("/api/v1", api_router)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn version() -> Json<Version> {
    Json(Version {
        version: VERSION.to_owned(),
    })
}
