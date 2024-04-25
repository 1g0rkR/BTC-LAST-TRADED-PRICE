use crate::application::BtcService;

#[derive(Clone)]
pub struct AppState {
    pub btc_service: BtcService,
}
