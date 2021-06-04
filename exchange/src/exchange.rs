use std::collections::HashMap;

use crate::{
    Client, ClientID, ExchangeError, Registry, Transaction, TransactionID, TransactionType,
};

/// An exchange keeps track of all transactions.
/// It is designed to always be in a valid state.
/// If a transaction is invalid, it will be rejected by the exchanged and an error will be returned.
#[derive(Debug)]
pub struct Exchange {
    /// The registry handles client lookup and registration
    registry: Registry,
    // We use a simple map as a datastore for accepted transations.
    // This does not scale to a lot of transactions of course. In a real-world
    // scenario, one could use an external datastore like Redis and sharding
    // based on the client id to handle transactions on a cluster of instances.
    // (See consistent hashing) One would also have to consider disk storage
    // for backups, rollups, and migrations.
    transactions: HashMap<TransactionID, Transaction>,
}

impl Exchange {
    /// Create a new, empty exchange
    pub fn new() -> Exchange {
        Exchange {
            registry: Registry::new(),
            transactions: HashMap::new(),
        }
    }

    /// Returns an iterator over all active clients in the exchange registry
    pub fn clients(&self) -> impl Iterator<Item = &Client> {
        self.registry.clients.values()
    }

    /// Retrieve a client from the exchange (if existing)
    pub fn get_client(&mut self, id: ClientID) -> Option<&Client> {
        self.registry.get(&id)
    }

    /// For some transactions the transaction id must be unique
    /// Check that the given id is available
    fn assert_id_available(&self, transaction: &Transaction) -> Result<(), ExchangeError> {
        if self.transactions.contains_key(&transaction.tx) {
            return Err(ExchangeError::InvalidTransaction(
                *transaction,
                "The transaction ID already exists".to_string(),
            ));
        }
        Ok(())
    }

    /// Look up a certain transaction
    fn get_tx(&self, transaction: &Transaction) -> Result<Transaction, ExchangeError> {
        match self.transactions.get(&transaction.tx) {
            Some(prev_tx) => Ok(*prev_tx),
            None => Err(ExchangeError::InvalidTransaction(
                *transaction,
                "The given transaction ID DOES NOT exist".to_string(),
            )),
        }
    }

    /// Commit a transaction to the exchange.
    ///
    /// ## Errors
    ///
    /// Returns error in case of an invalid transaction
    pub fn handle(&mut self, transaction: Transaction) -> Result<(), ExchangeError> {
        match transaction.ttype {
            TransactionType::Deposit(amount) => {
                self.assert_id_available(&transaction)?;
                self.transactions.insert(transaction.tx, transaction);
                let client = self.registry.get_mut(&transaction.client)?;
                client.total += amount;
                client.available += amount;
            }
            TransactionType::Withdraw(amount) => {
                self.assert_id_available(&transaction)?;
                self.transactions.insert(transaction.tx, transaction);
                let client = self.registry.get_mut(&transaction.client)?;
                if client.available < amount {
                    return Err(ExchangeError::InvalidTransaction(
                        transaction,
                        format!(
                        "Insufficient funds available for transaction. Available: {}, required: {}",
                        client.available, amount
                    ),
                    ));
                }
                client.total -= amount;
                client.available -= amount;
            }
            TransactionType::Dispute => {
                let prev_tx = self.get_tx(&transaction)?;
                let client = self.registry.get_mut(&transaction.client)?;
                match prev_tx.ttype {
                    TransactionType::Deposit(amount) | TransactionType::Withdraw(amount) => {
                        client.available -= amount;
                        client.held += amount;
                    }
                    _ => {
                        return Err(ExchangeError::InvalidTransaction(
                            transaction,
                            "Given transaction was not a deposit or withdrawal and thus has no amount".to_string(),
                        ));
                    }
                };
            }
            TransactionType::Resolve => {
                let prev_tx = self.get_tx(&transaction)?;
                if let Some(amount) = prev_tx.amount() {
                    let client = self.registry.get_mut(&transaction.client)?;
                    client.held -= amount;
                    client.available += amount;
                } else {
                    return Err(ExchangeError::InvalidTransaction(
                        transaction,
                        "No amount associated with transaction".to_string(),
                    ));
                }
            }
            TransactionType::Chargeback => {
                let prev_tx = self.get_tx(&transaction)?;
                if let Some(amount) = prev_tx.amount() {
                    let client = self.registry.get_mut(&transaction.client)?;
                    client.held -= amount;
                    client.total -= amount;
                    client.locked = true;
                } else {
                    return Err(ExchangeError::InvalidTransaction(
                        transaction,
                        "No amount associated with transaction".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Default for Exchange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_exchange {
    use crate::Amount;

    use super::*;

    #[test]
    fn test_deposit() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(1000, 0));
        assert_eq!(client.available, Amount::new(1000, 0));
        assert_eq!(client.held, Amount::new(0, 0));
        assert_eq!(client.locked, false);
    }

    #[test]
    fn test_withdraw_without_prior_deposit() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Withdraw(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_err());
        // Transaction failed, but client was created
        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(0, 0));
        assert_eq!(client.available, Amount::new(0, 0));
        assert_eq!(client.held, Amount::new(0, 0));
        assert_eq!(client.locked, false);
    }

    #[test]
    fn test_same_id_rejected() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_err());
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(2, 1, TransactionType::Withdraw(Amount::new(500, 0)));
        assert!(exchange.handle(tx).is_ok());
        // Transaction failed, but client was created
        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(500, 0));
        assert_eq!(client.available, Amount::new(500, 0));
        assert_eq!(client.held, Amount::new(0, 0));
        assert_eq!(client.locked, false);
    }

    #[test]
    fn test_dispute() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Dispute);
        assert!(exchange.handle(tx).is_ok());

        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(1000, 0));
        assert_eq!(client.available, Amount::new(0, 0));
        assert_eq!(client.held, Amount::new(1000, 0));
        assert_eq!(client.locked, false);
    }

    #[test]
    fn test_resolve() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Dispute);
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Resolve);
        assert!(exchange.handle(tx).is_ok());

        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(1000, 0));
        assert_eq!(client.available, Amount::new(1000, 0));
        assert_eq!(client.held, Amount::new(0, 0));
        assert_eq!(client.locked, false);
    }

    #[test]
    fn test_chargeback() {
        let mut exchange = Exchange::new();
        let tx = Transaction::new(1, 1, TransactionType::Deposit(Amount::new(1000, 0)));
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Dispute);
        assert!(exchange.handle(tx).is_ok());
        let tx = Transaction::new(1, 1, TransactionType::Chargeback);
        assert!(exchange.handle(tx).is_ok());

        let client = exchange.get_client(1).unwrap();
        assert_eq!(client.total, Amount::new(0, 0));
        assert_eq!(client.available, Amount::new(0, 0));
        assert_eq!(client.held, Amount::new(0, 0));
        assert_eq!(client.locked, true);
    }
}
