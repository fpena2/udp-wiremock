use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("invalid number packets received (expected {expected}, found {found})")]
    InvalidPacketsReceived { expected: usize, found: usize },
    #[error("invalid packet received: {0}")]
    InvalidPacketType(String),
    #[error("invalid mock state")]
    InvalidMockState,
}
