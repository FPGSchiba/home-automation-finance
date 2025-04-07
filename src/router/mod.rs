use axum::{
    extract::{Extension, Request},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use budget_categories::get_budget_category_router;
use budget_views::get_budget_view_router;
use budgets::get_budget_router;
use expense_categories::get_expense_category_router;
use expenses::get_expense_router;
use glob_match::glob_match;
use groups::get_group_router;
use repeating_expenses::get_repeating_expenses_router;
use reqwest::header::AUTHORIZATION;
use saving_goals::get_saving_goal_router;
use serde::{Deserialize, Serialize};
use std::{error::Error, str, sync::Arc};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utils::UserInformation;

use crate::{AppState, USER_API_URL};

mod budget_categories;
mod budget_views;
mod budgets;
mod expense_categories;
mod expenses;
mod groups;
mod repeating_expenses;
mod saving_goals;
mod utils;

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
    status: String,
    permissions: Vec<Permission>,
    #[serde(rename = "userId")]
    user_id: String,
}

async fn get_permissions_from_token(token: &str) -> Result<PermissionsResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(USER_API_URL.to_owned() + "/permissions/")
        .header(AUTHORIZATION, token)
        .send()
        .await?;
    let value: PermissionsResponse = res.json().await?;
    Ok(value)
}

async fn auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token)
            if token_is_valid(
                token,
                format!("/api/v1/finance{}/", request.uri().to_string()),
                request.method().to_string(),
            )
            .await =>
        {
            if let Ok(permissions_response) = get_permissions_from_token(token).await {
                request.extensions_mut().insert(UserInformation {
                    user_id: permissions_response.user_id,
                });
            }
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        Some(auth_header.to_str().unwrap())
    } else {
        None
    }
}

async fn token_is_valid(token: &str, uri: String, method: String) -> bool {
    match get_permissions_from_token(token).await {
        Ok(value) => {
            for permission in value.permissions.iter() {
                for route in permission.routes.iter() {
                    if glob_match(&route.path, &uri) && route.methods.contains(&method) {
                        return true;
                    }
                }
            }
            false
        }
        Err(error) => {
            println!("{}", error);
            false
        }
    }
}

async fn debug_auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    request.extensions_mut().insert(UserInformation {
        user_id: "6798dd5ddd620811a5cb8450".to_owned(),
    });
    let response = next.run(request).await;
    Ok(response)
}

pub fn get_router(is_debug: bool) -> Router {
    let group_router: Router = get_group_router();
    let expense_router = get_expense_router();
    let repeating_expense_router = get_repeating_expenses_router();
    let expense_category_router = get_expense_category_router();
    let budget_category_router = get_budget_category_router();
    let budget_router = get_budget_router();
    let budget_view_router = get_budget_view_router();
    let saving_goal_router = get_saving_goal_router();
    let mut api_router = Router::new()
        .nest("/groups", group_router)
        .nest("/expenses", expense_router)
        .nest("/repeating-expenses", repeating_expense_router)
        .nest("/expense-categories", expense_category_router)
        .nest("/budget-categories", budget_category_router)
        .nest("/budgets", budget_router)
        .nest("/budget-views", budget_view_router)
        .nest("/saving-goals", saving_goal_router);
    if !is_debug {
        api_router = api_router.route_layer(middleware::from_fn(auth));
    } else {
        api_router = api_router.route_layer(middleware::from_fn(debug_auth));
    }

    Router::new()
        .route("/api/v1/", get(version))
        .nest("/api/v1/", api_router)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn version(Extension(state): Extension<Arc<AppState>>) -> Json<Version> {
    let db = &state.db;
    Json(Version {
        version: VERSION.to_owned(),
    })
}
