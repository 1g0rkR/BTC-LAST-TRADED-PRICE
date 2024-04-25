use axum::http::StatusCode;
use axum::response::IntoResponse;

const INTERNAL_SERVER_ERROR_MSG: &str =
    "Oops! Server trouble at the moment. Please try again later";

#[derive(Debug)]
pub enum RetrieveBitcoinLtpErrors {
    InternalError,
}

impl From<anyhow::Error> for RetrieveBitcoinLtpErrors {
    fn from(_: anyhow::Error) -> Self {
        Self::InternalError
    }
}

impl IntoResponse for RetrieveBitcoinLtpErrors {
    fn into_response(self) -> axum::response::Response {
        match self {
            RetrieveBitcoinLtpErrors::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR_MSG).into_response()
            }
        }
    }
}
