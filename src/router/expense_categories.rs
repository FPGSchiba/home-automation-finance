use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_expense_category_router() -> Router {
    Router::new()
        .route("/", get(list_expense_categories))
        .route("/", post(create_expense_category))
        .route("/{expense_category_id}", get(get_expense_category))
        .route("/{expense_category_id}", put(update_expense_category))
        .route("/{expense_category_id}", delete(delete_expense_category))
}

async fn list_expense_categories() -> &'static str {
    "not implemented"
}

async fn create_expense_category() -> &'static str {
    "not implemented"
}

async fn get_expense_category(Path(expense_category_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_expense_category(Path(expense_category_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_expense_category(Path(expense_category_id): Path<String>) -> &'static str {
    "not implemented"
}
