use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_budget_router() -> Router {
    Router::new()
        .route("/", get(list_budgets))
        .route("/", post(create_budget))
        .route("/:budget_id", get(get_budget))
        .route("/:budget_id", put(update_budget))
        .route("/:budget_id", delete(delete_budget))
}

async fn list_budgets() -> &'static str {
    "not implemented"
}

async fn create_budget() -> &'static str {
    "not implemented"
}

async fn get_budget(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_budget(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_budget(Path(budget_id): Path<String>) -> &'static str {
    "not implemented"
}
