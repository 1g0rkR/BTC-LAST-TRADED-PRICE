mod app_state;
mod v1;

pub use app_state::AppState;
use axum::Router;

use self::v1::router_v1;

pub fn router(app_state: AppState) -> Router {
    Router::new().nest("/api", router_v1(app_state))
}
