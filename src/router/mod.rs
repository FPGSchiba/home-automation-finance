use axum::{routing::get, Json, Router};
use expenses::get_expense_router;
use groups::get_group_router;
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod expenses;
mod groups;

#[derive(Serialize)]
struct Version {
    version: String,
}

pub fn get_router() -> Router {
    let group_router = get_group_router();
    let expense_router = get_expense_router();

    Router::new()
        .route("/api/v1/", get(version))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .nest("/groups", group_router)
        .nest("/expenses", expense_router)
}

async fn version() -> Json<Version> {
    Json(Version {
        version: "0.1.0".to_owned(),
    })
}
