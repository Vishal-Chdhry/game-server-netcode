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
use game_server::{args::ServerArgs, version::VERSION};
use log::{error, warn};
use serde::Deserialize;
use std::{
    net::{SocketAddr, TcpStream},
    sync::{Arc, RwLock},
    vec,
};
use uuid::Uuid;

struct ServerState {
    args: ServerArgs,
    conns: Vec<TcpStream>,
}

type State = Arc<RwLock<ServerState>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = ServerArgs::parse();

    warn!("Starting server with {:?}", args);

    let mut conns = vec![];

    for server in &args.servers {
        match TcpStream::connect(server) {
            Ok(stream) => conns.push(stream),
            Err(e) => {
                error!(
                    "recieved an error establisheing a TCP connection to {} {}",
                    server, e
                );
            }
        }
    }

    let shared_state = Arc::new(RwLock::new(ServerState { conns, args }));

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
    version: Option<usize>,
    uuid: Option<Uuid>,
}

async fn find_server(state: State) -> String {
    return String::from("");
}

async fn handler(
    Query(params): Query<JoinParams>,
    Extension(_state): Extension<State>,
) -> Result<String, AppError> {
    if let Some(v) = params.version {
        if v != VERSION {
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
