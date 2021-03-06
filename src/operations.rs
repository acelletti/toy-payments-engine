#[cfg(test)]
mod tests;

use crate::ledger::Ledger;
use crate::tx_parser::TransactionParser;

/// Parse transactions from CSV and add them to the ledger
pub fn update_ledger_from_csv<R>(ledger: &mut Ledger, reader: &mut csv::Reader<R>)
where
    R: std::io::Read,
{
    let parser = TransactionParser::new(reader);

    for transaction in parser {
        ledger.process_transaction(&transaction)
    }
}

/// Write ledger to CSV
pub fn write_ledger_to_csv<W>(ledger: &Ledger, writer: &mut csv::Writer<W>) -> std::io::Result<()>
where
    W: std::io::Write,
{
    // serialize values in unsorted order
    for val in ledger.accounts() {
        writer.serialize(val)?;
    }
    writer.flush()
}
