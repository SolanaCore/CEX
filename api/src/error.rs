use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid kline interval `{0}` is not available. Available intervals: 1m, 5m, 15m, 1h, 1d, etc.")]
    InvalidKlineInterval(String),

    #[error("Invalid symbol `{0}` provided. Please check the symbol or see the supported symbols list.")]
    InvalidSymbol(String),

    #[error("Invalid sign-in credentials for username `{0}`. Please check your credentials.")]
    InvalidSigninCredentials(String),

    #[error("Invalid sign-up credentials. The username `{0}` already exists in the database.")]
    InvalidSignupCredentials(String),

    #[error("Invalid trade limit `{0}`. The maximum allowed limit is 1000 trades per symbol.")]
    InvalidLimit(u16),

    #[error("Invalid start time: `{0}`. The provided start time is in an invalid format.")]
    InvalidStartTime(String),

    #[error("Invalid end time `{0}`: The time is in the future. Please provide a valid end time that is not in the future.")]
    InvalidEndTime(String),

    #[error("The start and end times must be in RFC 3339 format (e.g., `2025-08-16T14:30:00Z`). Please ensure the format is correct.")]
    InvalidTimeFormat,

    #[error("Invalid date-time range: The start time `{0}` is after the end time `{1}`.")]
    StartTimeAfterEndTime(String, String),

    #[error("Invalid date-time: `{0}` is not a valid UTC time. Please provide a valid date-time in the RFC 3339 format.")]
    InvalidUTCDateTime(String),

    #[error("Unable to process the request due to a server issue. Please try again later.")]
    ServerError,

    #[error("Unknown error occurred. Please contact support.")]
    UnknownError,
}


