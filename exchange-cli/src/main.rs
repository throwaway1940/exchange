//! # Exchange CLI
//!
//! `exchange-cli` is a parser for files containing exchange transactions.  At
//! the moment only CSV files are accepted.  Invalid transactions get logged but
//! otherwise ignored as per the specification.
//!
//! The exchange-cli binary is just a wrapper around the [`exchange`] library. It
//! provides convenience functions for interacting with an exchange from from
//! the command-line
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod cli;
mod conversion;

use anyhow::Result;
use log::{error, warn};
use std::env;
use std::io;

const EXIT_NO_FILE: i32 = 1;
const EXIT_INVALID: i32 = 2;

fn main() -> Result<()> {
    env_logger::init();

    let path = env::args().nth(1).unwrap_or_else(|| {
        error!("Usage: cargo run -- transactions.csv > accounts.csv");
        std::process::exit(EXIT_NO_FILE);
    });

    if let Err(err) = cli::run(path, io::stdout()) {
        error!("Cannot handle input file: {:?}", err);
        std::process::exit(EXIT_INVALID);
    }

    Ok(())
}
