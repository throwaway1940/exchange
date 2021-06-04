use anyhow::{anyhow, Result};
use exchange::Amount;
use exchange::ClientID;
use exchange::Transaction;
use exchange::TransactionID;
use exchange::TransactionType;
use serde::Deserialize;
use std::convert::TryFrom;

/// A helper struct to read data from the CSV file.
/// The conversion to an actual `Transaction` is done in a separate step.
/// That's because there is some verification needed to check that the transaction
/// has an amount (in case of deposit or withdrawal) or not (otherwise)
/// This struct is not part of the library as it is a special conversion struct
/// dependent on the program input.
#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    #[serde(alias = "type")]
    ttype: String,
    client: ClientID,
    tx: TransactionID,
    amount: Option<Amount>,
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = anyhow::Error;

    fn try_from(raw: RawTransaction) -> Result<Self, Self::Error> {
        Ok(Transaction {
            tx: raw.tx,
            client: raw.client,
            ttype: parse_ttype(raw.ttype, raw.amount)?,
        })
    }
}

/// Helper function to parse the transaction type
fn parse_ttype(ttype: String, amount: Option<Amount>) -> Result<TransactionType> {
    let ttype = match (ttype.as_str(), amount) {
        ("deposit", Some(amount)) => TransactionType::Deposit(amount),
        // The docs mention "withdraw" and "withdrawal", so let's accept both
        ("withdraw", Some(amount)) | ("withdrawal", Some(amount)) => {
            TransactionType::Withdraw(amount)
        }
        ("dispute", None) => TransactionType::Dispute,
        ("resolve", None) => TransactionType::Resolve,
        ("chargeback", None) => TransactionType::Chargeback,
        _ => {
            return Err(anyhow!(
                "Unsupported transaction: type {}, amount {:?}",
                ttype,
                amount
            ))
        }
    };
    Ok(ttype)
}
