use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_expense_router() -> Router {
    Router::new()
        .route("/", get(list_expenses))
        .route("/", post(create_expense))
        .route("/:expense_id", get(get_expense))
        .route("/:expense_id", put(update_expense))
        .route("/:expense_id", delete(delete_expense))
        .route("/:expense_id", post(assign_members))
}

async fn list_expenses() -> &'static str {
    "not implemented"
}

async fn create_expense() -> &'static str {
    "not implemented"
}

async fn get_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_expense(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn assign_members(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}
