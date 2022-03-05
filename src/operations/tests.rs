use super::*;
use crate::accounts::Account;

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
        *ledger.get_or_create(1),
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
        *ledger.get_or_create(2),
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
        *ledger.get_or_create(3),
        Account {
            client: 3,
            total: 0.0,
            available: 0.0,
            held: 0.0,
            locked: true,
        }
    );
}
