use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_repeating_expenses_router() -> Router {
    Router::new()
        .route("/", get(list_repeating_expenses))
        .route("/", post(create_repeating_expense))
        .route("/:expense_id", get(get_repeating_expense))
        .route("/:expense_id", put(update_repeating_expense))
        .route("/:expense_id", delete(delete_repeating_expense))
}

async fn list_repeating_expenses() -> &'static str {
    "not implemented"
}

async fn create_repeating_expense() -> &'static str {
    "not implemented"
}

async fn get_repeating_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_repeating_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_repeating_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}
