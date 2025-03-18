use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketNotFound { id: u64 },
    AuthFailNoTokenProvided,
    AuthTokenWrongFormat,
    DatabaseConnectionError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<SqlxError> for Error {
    fn from(_: SqlxError) -> Self {
        Error::DatabaseConnectionError
    }
}
