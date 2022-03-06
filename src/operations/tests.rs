use super::*;
use crate::models::account::Account;
use crate::models::transaction::{Transaction, TransactionType};

// sample CSV data to parse
const SAMPLE_DATA: &str = "\
type,client,tx,amount
deposit,1,1,50
withdrawal,1,2,12.50
deposit,2,3,25
dispute,2,3,
resolve,2,3,
deposit,3,4,15
dispute,3,4,
chargeback,3,4,
";

// test that all transactions types are parsed correctly from CSV format
#[test]
fn transaction_are_parsed_correctly_into_ledger() {
    let mut reader = csv::Reader::from_reader(SAMPLE_DATA.as_bytes());
    let mut ledger = Ledger::new();

    // parse transactions
    update_ledger_from_csv(&mut ledger, &mut reader);

    // deposit - withdrawal
    assert_eq!(
        *ledger.get(1).unwrap(),
        Account {
            client: 1,
            total: 37.5,
            available: 37.5,
            held: 0.0,
            locked: false,
        }
    );

    // dispute with resolution
    assert_eq!(
        *ledger.get(2).unwrap(),
        Account {
            client: 2,
            total: 25.0,
            available: 25.0,
            held: 0.0,
            locked: false,
        }
    );

    // dispute with chargeback
    assert_eq!(
        *ledger.get(3).unwrap(),
        Account {
            client: 3,
            total: 0.0,
            available: 0.0,
            held: 0.0,
            locked: true,
        }
    );
}

// expected CSV data with precision of four places past the decimal
const EXPECTED_CSV: &str = "\
client,available,held,total,locked
1,1.2346,0.0000,1.2346,false
";

// test that all operation types are parsed correctly from CSV format
#[test]
fn ledger_exports_to_csv_correctly() {
    let mut ledger = Ledger::new();
    ledger.process_transaction(&Transaction {
        transaction_type: TransactionType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(1.23456789),
    });

    let mut writer = csv::Writer::from_writer(vec![]);
    write_ledger_to_csv(&mut ledger, &mut writer).unwrap();

    let data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    assert_eq!(data, EXPECTED_CSV);
}
