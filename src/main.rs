use router::get_router;
use tokio::net::TcpListener;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .compact()
        .init();

    let api_router = get_router();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, api_router).await.unwrap();
}
