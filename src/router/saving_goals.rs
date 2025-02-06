use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_saving_goal_router() -> Router {
    Router::new()
        .route("/", get(list_saving_goals))
        .route("/", post(create_saving_goal))
        .route("/{goal_id}", get(get_saving_goal))
        .route("/{goal_id}", put(update_saving_goal))
        .route("/{goal_id}", delete(delete_saving_goal))
}

async fn list_saving_goals() -> &'static str {
    "not implemented"
}

async fn create_saving_goal() -> &'static str {
    "not implemented"
}

async fn get_saving_goal(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_saving_goal(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_saving_goal(Path(expense_id): Path<String>) -> &'static str {
    "not implemented"
}
