use axum::{routing::get, Router};

use super::AppState;

mod errors;
mod handlers;

pub fn router_v1(app_state: AppState) -> Router {
    Router::new().nest("/v1", router(app_state))
}

fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::bitcoin_ltp))
        .with_state(app_state)
}
