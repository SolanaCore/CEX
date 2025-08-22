use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClickhouseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query execution error: {0}")]
    QueryExecutionError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}