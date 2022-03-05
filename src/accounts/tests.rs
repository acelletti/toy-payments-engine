use super::*;

// expected CSV data with precision of four places past the decimal
const EXPECTED_CSV: &str = "\
client,available,held,total,locked
1,0.0000,0.0000,1.2346,false
";

// test that all operation types are parsed correctly from CSV format
#[test]
fn ledger_exports_to_csv_correctly() {
    let mut ledger = Ledger::new();
    ledger.get_or_create(1);

    // get account & set long float as total
    let mut account = ledger.get_or_create(1);
    account.total = 1.23456789;

    let mut writer = csv::Writer::from_writer(vec![]);
    ledger.write_to_csv(&mut writer).unwrap();

    let data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    assert_eq!(data, EXPECTED_CSV);
}

const CLIENT_ID: u16 = 1;

struct TxHandler {
    tx_counter: u32,
}

impl TxHandler {
    pub fn new() -> Self {
        Self { tx_counter: 1 }
    }

    fn process_generic(
        &mut self,
        ledger: &mut Ledger,
        transaction_type: TransactionType,
        transaction_id: Option<u32>,
        amount: Option<f32>,
    ) -> Option<u32> {
        let tx = transaction_id.unwrap_or(*&self.tx_counter);
        ledger.process_transaction(&Transaction {
            transaction_type,
            client: CLIENT_ID,
            tx,
            amount,
        });
        // increase tx id if one not provided
        if transaction_id.is_none() {
            self.tx_counter += 1;
            return Some(tx);
        }
        None
    }

    pub fn process_deposit(&mut self, ledger: &mut Ledger, amount: f32) -> u32 {
        self.process_generic(ledger, TransactionType::Deposit, None, Some(amount))
            .unwrap()
    }

    pub fn process_withdrawal(&mut self, ledger: &mut Ledger, amount: f32) -> u32 {
        self.process_generic(ledger, TransactionType::Withdrawal, None, Some(amount))
            .unwrap()
    }

    pub fn process_dispute(&mut self, ledger: &mut Ledger, transaction_id: u32) {
        self.process_generic(ledger, TransactionType::Dispute, Some(transaction_id), None);
    }

    pub fn process_resolve(&mut self, ledger: &mut Ledger, transaction_id: u32) {
        self.process_generic(ledger, TransactionType::Resolve, Some(transaction_id), None);
    }

    pub fn process_chargeback(&mut self, ledger: &mut Ledger, transaction_id: u32) {
        self.process_generic(
            ledger,
            TransactionType::Chargeback,
            Some(transaction_id),
            None,
        );
    }
}

fn assert_account_state(ledger: &mut Ledger, total: f32, available: f32, held: f32, locked: bool) {
    assert_eq!(
        *ledger.get_or_create(CLIENT_ID),
        Account {
            client: CLIENT_ID,
            total,
            available,
            held,
            locked,
        }
    );
}

#[test]
fn deposit_is_processed_correctly() {
    let mut ledger = Ledger::new();
    let mut handler = TxHandler::new();
    handler.process_deposit(&mut ledger, 12.34);

    // check that funds are deposited correctly
    assert_account_state(&mut ledger, 12.34, 12.34, 0.0, false);
}

#[test]
fn withdrawal_is_processed_correctly() {
    let mut ledger = Ledger::new();

    let mut handler = TxHandler::new();
    handler.process_deposit(&mut ledger, 30.0);
    handler.process_withdrawal(&mut ledger, 15.0);

    // check that account balance is correct
    assert_account_state(&mut ledger, 15.0, 15.0, 0.0, false);

    // empty account funds
    handler.process_withdrawal(&mut ledger, 15.0);

    // check that account is empty
    assert_account_state(&mut ledger, 0.0, 0.0, 0.0, false);

    handler.process_deposit(&mut ledger, 5.0);
    // this withdrawal should be ignored as there aren't enough funds
    handler.process_withdrawal(&mut ledger, 50.0);

    // check that deposit was successful but withdrawal failed
    assert_account_state(&mut ledger, 5.0, 5.0, 0.0, false);
}

#[test]
fn dispute_is_processed_correctly() {
    let mut ledger = Ledger::new();

    let mut handler = TxHandler::new();
    let tx1 = &handler.process_deposit(&mut ledger, 20.0);
    handler.process_deposit(&mut ledger, 30.0);
    assert_account_state(&mut ledger, 50.0, 50.0, 0.0, false);

    // dispute TX 1
    handler.process_dispute(&mut ledger, *tx1);

    // check that funds from deposit were held
    assert_account_state(&mut ledger, 50.0, 30.0, 20.0, false);

    // dispute TX 1 again, this should be ignored
    handler.process_dispute(&mut ledger, *tx1);

    // dispute invalid TX ID, this should be ignored as well
    handler.process_dispute(&mut ledger, 100);

    // check that account state is unchanged
    assert_account_state(&mut ledger, 50.0, 30.0, 20.0, false);
}

#[test]
fn resolve_dispute_is_processed_correctly() {
    let mut ledger = Ledger::new();

    let mut handler = TxHandler::new();
    let tx1 = &handler.process_deposit(&mut ledger, 20.0);
    handler.process_deposit(&mut ledger, 30.0);

    // dispute TX 1
    handler.process_dispute(&mut ledger, *tx1);

    // check that funds from deposit were held
    assert_account_state(&mut ledger, 50.0, 30.0, 20.0, false);

    // resolve dispute for TX 1
    handler.process_resolve(&mut ledger, *tx1);

    // check that funds were released
    assert_account_state(&mut ledger, 50.0, 50.0, 0.0, false);

    // resolve dispute for TX 1 again, should be ignored
    handler.process_resolve(&mut ledger, *tx1);

    // resolve dispute for invalid TX ID, should be ignored
    handler.process_resolve(&mut ledger, 100);

    // check that account state is unchanged
    assert_account_state(&mut ledger, 50.0, 50.0, 0.0, false);
}

#[test]
fn chargeback_is_processed_correctly() {
    let mut ledger = Ledger::new();

    let mut handler = TxHandler::new();
    let tx1 = &handler.process_deposit(&mut ledger, 20.0);
    handler.process_deposit(&mut ledger, 30.0);

    // dispute TX 1
    handler.process_dispute(&mut ledger, *tx1);

    // check that funds from deposit were held
    assert_account_state(&mut ledger, 50.0, 30.0, 20.0, false);

    // chargeback for TX 1
    handler.process_chargeback(&mut ledger, *tx1);

    // check that funds were charged back and account is locked
    assert_account_state(&mut ledger, 30.0, 30.0, 0.0, true);
}
