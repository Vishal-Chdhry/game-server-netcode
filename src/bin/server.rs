use anyhow::Result;
use axum::{
    extract::Extension,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use game_server::args::ServerArgs;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};

type State = Arc<ServerArgs>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = ServerArgs::parse();
    let shared_state = Arc::new(args);

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
    Extension(state): Extension<State>,
) -> Result<String, AppError> {
    if let Some(v) = params.version {
        if v != state.version {
            return Err(anyhow::anyhow!("version out of date -- please update").into());
        }
    } else {
        return Err(anyhow::anyhow!("please provide a version").into());
    }
    if let None = params.uuid {
        return Err(anyhow::anyhow!("please provide a uuid").into());
    }
    return Ok(format!("count is {:?}", params));
}
// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
