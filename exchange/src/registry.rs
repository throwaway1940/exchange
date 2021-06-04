use crate::{Client, ClientID, ExchangeError};
use std::collections::HashMap;

/// Stores information of all clients of the exchange
/// It handles client lookup and registration
#[derive(Debug, Clone)]
pub struct Registry {
    /// Map of clients active in the registry
    pub clients: HashMap<ClientID, Client>,
}

impl Registry {
    /// Create a new, empty registry of clients
    pub fn new() -> Self {
        let clients = HashMap::new();
        Registry { clients }
    }

    /// Get mutable information for client with given id
    /// Note that this will always return a client (and not an option):
    /// If a client doesn't exist, it creates a new record
    /// If a client is locked, an error is returned as the client can no longer be modified.
    /// Use `get` to get a read-only state in this case.
    pub fn get_mut(&mut self, id: &ClientID) -> Result<&mut Client, ExchangeError> {
        let client = self.clients.entry(*id).or_insert(Client::new(*id));
        if client.locked {
            return Err(ExchangeError::Locked(*client));
        }
        Ok(client)
    }

    /// Get information for client with given id (if existing)
    pub fn get(&mut self, id: &ClientID) -> Option<&Client> {
        self.clients.get(&id)
    }

    /// Register client
    pub fn register(&mut self, client: Client) -> Option<Client> {
        self.clients.insert(client.id, client)
    }
}
