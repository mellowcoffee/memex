#![allow(clippy::needless_for_each)]
// #![allow(dead_code)]

mod db;
mod error;
mod files;
mod model;
mod parser;
mod routing;
mod templates;

use axum::{
    Router,
    extract::State,
    http::{StatusCode, header},
    routing::get,
};

use crate::{
    files::{FILES_DIR_PATH, read_files_from_dir},
    model::Wiki,
};
use crate::routing::pages;

#[derive(Clone)]
struct AppState {
    wiki: Wiki,
}

#[tokio::main]
async fn main() {
    let files = read_files_from_dir(FILES_DIR_PATH).unwrap();
    let wiki = Wiki::init_from_files(files).await.unwrap();

    let state = AppState { wiki };
    let app = Router::new()
        .route("/", get(async || "Hello world!"))
        .route(
            "/style.css",
            get(async || {
                let stylesheet = std::fs::read_to_string("./static/style.css");
                match stylesheet {
                    Err(e) => panic!("FATAL: Reading the stylesheet failed: {e}"),
                    Ok(s) => (StatusCode::OK, [(header::CONTENT_TYPE, "text/css")], s),
                }
            }),
        )
        .merge(pages::routes(State(state)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
