use serde::{Serialize, Serializer};

use crate::Amount;

/// Precision of output fractional
pub(crate) const PRECISION: u32 = 4;

/// ID of a client
/// Make client ID a separate type to allow for future upgrades
pub type ClientID = u16;

fn serialize_amount<S>(amount: &Amount, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&amount.round_dp(PRECISION).to_string())
}

/// Encapsulates the state of a single client
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub struct Client {
    /// Unique ID
    #[serde(rename(serialize = "client"))]
    pub id: ClientID,
    /// Amount available for transactions (i.e. not locked by disputes)
    #[serde(serialize_with = "serialize_amount")]
    pub available: Amount,
    /// The total funds that are available or held. This should be equal to available + held
    #[serde(serialize_with = "serialize_amount")]
    pub held: Amount,
    /// Total amount in account
    #[serde(serialize_with = "serialize_amount")]
    pub total: Amount,
    /// Whether the account is locked. An account is locked if a charge back occurs
    pub locked: bool,
}

impl Client {
    /// Create a new client with the given ID
    pub fn new(id: ClientID) -> Self {
        Self {
            id,
            available: Amount::default(),
            held: Amount::default(),
            total: Amount::default(),
            locked: false,
        }
    }
}
