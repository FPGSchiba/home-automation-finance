use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn get_group_router() -> Router {
    Router::new()
        .route("/", get(list_groups))
        .route("/", post(create_group))
        .route("/:group_id", get(get_group))
        .route("/:group_id", put(update_group))
        .route("/:group_id", delete(delete_group))
        .route("/:group_id", post(assign_members))
}

async fn list_groups() -> &'static str {
    "not implemented"
}

async fn create_group() -> &'static str {
    "not implemented"
}

async fn get_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn assign_members(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}
