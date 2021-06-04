use std::{convert::TryInto, io, path::Path};

use anyhow::Result;
use exchange::{Exchange, Transaction};
use log::{debug, warn};

use crate::conversion::RawTransaction;

pub fn run<P: AsRef<Path>, W: io::Write>(input: P, writer: W) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .quoting(false)
        .delimiter(b',')
        .double_quote(false)
        .has_headers(true)
        .comment(Some(b'#'))
        .from_path(input.as_ref())?;

    let mut exchange = Exchange::new();

    for result in reader.deserialize() {
        let raw: RawTransaction = if let Ok(raw) = result { raw } else { continue };

        let transaction: Transaction = match raw.try_into() {
            Err(e) => {
                debug!("Invalid transaction {}", e);
                continue;
            }
            Ok(t) => t,
        };
        if let Err(e) = exchange.handle(transaction) {
            warn!("Transaction failed: {}", e);
        }
    }

    let mut writer = csv::Writer::from_writer(writer);
    for client in exchange.clients() {
        writer.serialize(client)?;
    }
    Ok(())
}
