use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_budget_view_router() -> Router {
    Router::new()
        .route("/", get(list_budget_views))
        .route("/", post(create_budget_view))
        .route("/{view_id}", get(get_budget_view))
        .route("/{view_id}", put(update_budget_view))
        .route("/{view_id}", delete(delete_budget_view))
}

async fn list_budget_views() -> &'static str {
    "not implemented"
}

async fn create_budget_view() -> &'static str {
    "not implemented"
}

async fn get_budget_view(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_budget_view(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_budget_view(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}
