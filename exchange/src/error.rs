use thiserror::Error;

use crate::{Client, Transaction};

/// Possible errors when interacting with the exchange
#[derive(Error, Debug, PartialEq)]
pub enum ExchangeError {
    /// Error during amount conversion to internal format
    #[error("Amount conversion failed. Expected fraction with a precision of up to four places past the decimal, got `{0}`: `{1}`")]
    InvalidAmount(String, String),
    /// Error while validating a transaction
    #[error("The given transaction is invalid: `{0:?}`. Transaction: `{1:?}`")]
    InvalidTransaction(Transaction, String),
    /// If a client is locked it can no longer be modified
    #[error("The client is locked and immutable. `{0:?}`")]
    Locked(Client),
}
