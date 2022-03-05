#[cfg(test)]
mod tests;

use crate::accounts::Ledger;
use crate::transactions::TransactionParser;

// parse transactions from CSV and add them to the ledger
pub fn update_ledger_from_csv<R>(ledger: &mut Ledger, reader: &mut csv::Reader<R>)
where
    R: std::io::Read,
{
    let parser = TransactionParser::new(reader);

    for transaction in parser {
        ledger.process_transaction(&transaction)
    }
}
