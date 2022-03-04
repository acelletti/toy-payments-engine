#[cfg(test)]
mod tests;

use serde::Serializer;
use std::collections::HashMap;

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
    client: u16,
    #[serde(serialize_with = "fixed_width")]
    available: f32,
    #[serde(serialize_with = "fixed_width")]
    held: f32,
    #[serde(serialize_with = "fixed_width")]
    total: f32,
    locked: bool,
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
    accounts: HashMap<u16, Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    // get or create account for given client ID
    pub fn get_or_create(&mut self, client_id: u16) -> &mut Account {
        self.accounts
            .entry(client_id)
            .or_insert(Account::new(client_id))
    }

    pub fn write_to_csv<W>(&self, writer: &mut csv::Writer<W>) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        // serialize values in unsorted order
        for val in self.accounts.values() {
            writer.serialize(val)?;
        }
        writer.flush()
    }
}
