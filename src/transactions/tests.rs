use super::*;

// sample CSV data with one of each type
const SAMPLE_DATA: &str = "\
type,client,tx,amount
deposit,1,1,1.0
withdrawal,1,4,1.5
dispute,1,1,
resolve,1,1,
chargeback,1,1,
";

const EXPECTED_TRANSACTIONS: [Transaction; 5] = [
    Transaction {
        transaction_type: TransactionType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(1.0),
    },
    Transaction {
        transaction_type: TransactionType::Withdrawal,
        client: 1,
        tx: 4,
        amount: Some(1.5),
    },
    Transaction {
        transaction_type: TransactionType::Dispute,
        client: 1,
        tx: 1,
        amount: None,
    },
    Transaction {
        transaction_type: TransactionType::Resolve,
        client: 1,
        tx: 1,
        amount: None,
    },
    Transaction {
        transaction_type: TransactionType::Chargeback,
        client: 1,
        tx: 1,
        amount: None,
    },
];

// test that all transactions types are parsed correctly from CSV format
#[test]
fn transactions_parser_returns_correct_operations() {
    let mut reader = csv::Reader::from_reader(SAMPLE_DATA.as_bytes());

    let parser = TransactionParser::new(&mut reader);

    assert_eq!(
        parser.collect::<Vec<Transaction>>(),
        Vec::from(EXPECTED_TRANSACTIONS)
    );
}
