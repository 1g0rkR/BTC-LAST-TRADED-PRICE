use axum::{extract::State, response::IntoResponse, Json};

use crate::infrastructure::api::AppState;

use super::errors::RetrieveBitcoinLtpErrors;

pub async fn bitcoin_ltp(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, RetrieveBitcoinLtpErrors> {
    tracing::info!("Request bitcoin pairs LTP");
    Ok(Json(app_state.btc_service.get_bitcoin_ltp().await?))
}
