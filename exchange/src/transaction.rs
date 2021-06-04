use crate::{Amount, ClientID};

/// ID of a single transaction. It is unique across the entire exchange.
/// Make transaction ID a separate type to allow for future upgrades
pub type TransactionID = u32;

/// Types of transactions accepted on the exchange
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransactionType {
    /// A deposit is a credit to the client's asset account, meaning it should
    /// increase the available and total funds of the client account
    Deposit(Amount),
    /// A withdraw is a debit to the client's asset account, meaning it should decrease the available and total funds of the client account
    Withdraw(Amount),
    /// A dispute represents a client's claim that a transaction was erroneous
    /// and should be reversed.
    Dispute,
    /// A resolve represents a resolution to a dispute, releasing the associated
    /// held funds. Funds that were previously disputed are no longer disputed.
    /// This means that the clients held funds should decrease by the amount no
    /// longer disputed, their available funds should increase by the amount no
    /// longer disputed, and their total funds should remain the same.
    Resolve,
    /// A chargeback is the final state of a dispute and represents the client
    /// reversing a transaction. Funds that were held have now been withdrawn.
    /// This means that the clients held funds and total funds should decrease
    /// by the amount previously disputed. If a chargeback occurs the client's
    /// account should be immediately frozen.
    Chargeback,
}

/// Transactions contain all necessary information of a single transaction on
/// the exchange
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transaction {
    /// ID of transaction
    pub tx: TransactionID,
    /// Client ID for transaction
    pub client: ClientID,
    /// Transaction type (with optional amount)
    pub ttype: TransactionType,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(tx: TransactionID, client: ClientID, ttype: TransactionType) -> Self {
        Self { tx, client, ttype }
    }

    /// Return the amount of the transaction (if any)
    pub fn amount(&self) -> Option<Amount> {
        match self.ttype {
            TransactionType::Deposit(amount) | TransactionType::Withdraw(amount) => Some(amount),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let transaction = Transaction {
            tx: 1,
            client: 2,
            ttype: TransactionType::Deposit(Amount::new(100, 0)),
        };
        assert_eq!(transaction.tx, 1);
        assert_eq!(transaction.client, 2);
        assert!(matches!(transaction.ttype, TransactionType::Deposit(_)));
    }
}
