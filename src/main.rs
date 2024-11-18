use axum::{routing::get, Router};
use tokio::net::TcpListener;

use std::net::SocketAddr;

use tower_http::trace::{self, TraceLayer};

use tracing::Level;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .pretty()
        .init();

    let app = Router::new().route("/", get(hello_world)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello World!"
}
