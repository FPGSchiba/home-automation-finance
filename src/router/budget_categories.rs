use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_budget_category_router() -> Router {
    Router::new()
        .route("/", get(list_budget_categories))
        .route("/", post(create_budget_category))
        .route("/{budget_category_id}", get(get_budget_category))
        .route("/{budget_category_id}", put(update_budget_category))
        .route("/{budget_category_id}", delete(delete_budget_category))
}

async fn list_budget_categories() -> &'static str {
    "not implemented"
}

async fn create_budget_category() -> &'static str {
    "not implemented"
}

async fn get_budget_category(Path(budget_category_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_budget_category(Path(budget_category_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_budget_category(Path(budget_category_id): Path<String>) -> &'static str {
    "not implemented"
}
