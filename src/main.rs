extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod ledger;
mod models;
mod operations;
mod tx_parser;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv_file_path = std::env::args_os()
        .nth(1)
        .expect("Usage cargo run -- <csv_file>");

    let mut ledger = ledger::Ledger::new();

    // update ledger with transactions from CSV file
    let mut reader = csv::Reader::from_path(csv_file_path)?;
    operations::update_ledger_from_csv(&mut ledger, &mut reader);

    // export ledger as CSV to stdout
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    operations::write_ledger_to_csv(&ledger, &mut writer)?;

    Ok(())
}
