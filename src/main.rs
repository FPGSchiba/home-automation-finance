use axum::{routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};

use tracing::Level;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
struct Version {
    version: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .compact()
        .init();

    let api_router = Router::new().route("/api/v1/", get(version)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, api_router).await.unwrap();
}

async fn version() -> Json<Version> {
    Json(Version {
        version: "0.1.0".to_owned(),
    })
}
