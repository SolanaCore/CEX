use thiserror::Error;
use sqlx::Error as SqlxError;
use std::{io, env::VarError};

#[derive(Error, Debug)]
pub enum ClickhouseError {
    /// Database-related errors (connection, query, pool, etc.)
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    /// Environment variable issues (missing or invalid values)
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] VarError),

    /// IO errors (dotenv, config files, network streams, etc.)
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Query build errors with extra context
    #[error("Query error: {query}, reason: {reason}")]
    Query {
        query: String,
        reason: String,
    },

    /// Data validation issues
    #[error("Validation error: {0}")]
    Validation(String),

    /// Pool initialization / connection issues
    #[error("Pool error: {0}")]
    Pool(String),

    /// Unknown / catch-all errors
    #[error("Unknown error: {0}")]
    Unknown(String),
}
