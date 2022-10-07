use anyhow::Result;
use axum::{extract::Query, routing::get, Extension, Router};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

struct State {
    pub count: AtomicUsize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let shared_state = Arc::new(State {
        count: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/join", get(handler))
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JoinParams {
    version: Option<u8>,
    uuid: Option<String>,
}
async fn handler(
    Query(params): Query<JoinParams>,
    Extension(state): Extension<Arc<State>>,
) -> String {
    let count = state.count.fetch_add(1, Ordering::Relaxed);
    return format!("count is {} {:?}", count, params);
}
