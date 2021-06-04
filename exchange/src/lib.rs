//! `exchange` is a library for creating and managing an exchange.
#![warn(clippy::all, clippy::pedantic)]
#![warn(
    absolute_paths_not_starting_with_crate,
    invalid_html_tags,
    missing_copy_implementations,
    missing_debug_implementations,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    variant_size_differences,
    clippy::missing_const_for_fn
)]
#![deny(anonymous_parameters, macro_use_extern_crate, pointer_structural_match)]
#![deny(missing_docs)]

mod amount;
mod client;
mod error;
mod exchange;
mod registry;
mod transaction;

pub use crate::exchange::Exchange;
pub use amount::Amount;
pub use client::{Client, ClientID};
pub use error::ExchangeError;
pub use registry::Registry;
pub use transaction::{Transaction, TransactionID, TransactionType};
