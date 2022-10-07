use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use anyhow::Result;
use axum::{routing::get, Extension, Router};

struct State {
    pub count: AtomicUsize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let shared_state = Arc::new(State {
        count: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_state));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 42070));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    return Ok(());
}

async fn handler(Extension(state): Extension<Arc<State>>) -> String {
    let count = state.count.fetch_add(1, Ordering::Relaxed);
    return format!("count is {}", count);
}
