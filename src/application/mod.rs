mod services;

use std::net::SocketAddr;

pub use services::btc_service::BtcService;

use crate::infrastructure::api::{router, AppState};

pub async fn run_application(addr: impl Into<SocketAddr>) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr: SocketAddr = addr.into();

    let app_state: AppState = AppState {
        btc_service: BtcService::new()?,
    };

    let app = router(app_state);

    tracing::info!("Server running on port {} ...", addr.port());

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
