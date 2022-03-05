#[cfg(test)]
mod tests;

use crate::transactions::Transaction;
use crate::transactions::TransactionType;

use serde::Serializer;
use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;

// custom serializer for account floats that need to be output
// with precision of four places past the decimal (from the specs)
fn fixed_width<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{:.4}", value))
}

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Account {
    pub client: u16,
    #[serde(serialize_with = "fixed_width")]
    pub available: f32,
    #[serde(serialize_with = "fixed_width")]
    pub held: f32,
    #[serde(serialize_with = "fixed_width")]
    pub total: f32,
    pub locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

pub struct Ledger {
    // map of <Client ID, Account>
    accounts: HashMap<u16, Account>,

    // map of <Transaction ID, amount>
    deposits: HashMap<u32, f32>,

    // keep track of disputed transactions
    disputes: HashSet<u32>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            deposits: HashMap::new(),
            disputes: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, client_id: u16) -> Option<&Account> {
        self.accounts.get(&client_id)
    }

    pub fn accounts(&self) -> hash_map::Values<u16, Account> {
        self.accounts.values()
    }

    pub fn process_transaction(&mut self, transaction: &Transaction) {
        match transaction.transaction_type {
            TransactionType::Deposit => self.process_deposit(
                &transaction.client,
                &transaction.tx,
                &transaction.amount.expect("Invalid deposit data."),
            ),
            TransactionType::Withdrawal => self.process_withdrawal(
                &transaction.client,
                &transaction.amount.expect("Invalid withdrawal data."),
            ),
            TransactionType::Dispute => self.process_dispute(&transaction.client, &transaction.tx),
            TransactionType::Resolve => {
                self.process_resolve_dispute(&transaction.client, &transaction.tx)
            }
            TransactionType::Chargeback => {
                self.process_chargeback(&transaction.client, &transaction.tx)
            }
        }
    }

    // get or create account for given client ID
    fn get_or_create(&mut self, client_id: u16) -> &mut Account {
        self.accounts
            .entry(client_id)
            .or_insert(Account::new(client_id))
    }

    fn process_deposit(&mut self, client_id: &u16, transaction_id: &u32, amount: &f32) {
        let account = self.get_or_create(*client_id);

        account.total += amount;
        account.available += amount;

        self.deposits.insert(*transaction_id, *amount);
    }

    fn process_withdrawal(&mut self, client_id: &u16, amount: &f32) {
        let account = self.get_or_create(*client_id);

        if account.available >= *amount {
            account.available -= amount;
            account.total -= amount;
        }
    }

    fn process_dispute(&mut self, client_id: &u16, transaction_id: &u32) {
        if let Some(amount) = self.deposits.get(transaction_id).cloned() {
            if !self.disputes.contains(transaction_id) {
                let account = self.get_or_create(*client_id);
                account.available -= *&amount;
                account.held += *&amount;
                self.disputes.insert(*transaction_id);
            }
        }
    }

    fn process_resolve_dispute(&mut self, client_id: &u16, transaction_id: &u32) {
        if let Some(amount) = self.deposits.get(transaction_id).cloned() {
            if self.disputes.contains(transaction_id) {
                let account = self.get_or_create(*client_id);
                account.available += *&amount;
                account.held -= *&amount;
                self.disputes.remove(transaction_id);
            }
        }
    }

    fn process_chargeback(&mut self, client_id: &u16, transaction_id: &u32) {
        if let Some(amount) = self.deposits.get(transaction_id).cloned() {
            if self.disputes.contains(transaction_id) {
                let account = self.get_or_create(*client_id);
                account.total -= *&amount;
                account.held -= *&amount;
                account.locked = true;
                self.disputes.remove(transaction_id);
            }
        }
    }
}
