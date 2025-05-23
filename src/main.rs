mod db;
mod router;

use std::env;
use std::sync::Arc;

use axum::extract::Extension;
use db::errors::DBError;
use db::DB;
use dotenv::dotenv;
use router::get_router;
use tokio::net::TcpListener;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static USER_API_URL: &str = "http://localhost:8080/api/v1";

pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), DBError> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .compact()
        .init();

    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let mut is_debug = false;
    if args.len() > 1 && args[1] == "debug" {
        is_debug = true;
        tracing::info!("Debug mode enabled");
    }

    let db = DB::init().await?;
    let api_router = get_router(is_debug).layer(Extension(Arc::new(AppState { db })));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, api_router).await.unwrap();
    Ok(())
}
