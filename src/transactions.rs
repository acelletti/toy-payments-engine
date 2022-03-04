#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    client: u16,
    tx: u32,
    amount: Option<f32>,
}

pub struct TransactionParser<'a, R>
where
    R: std::io::Read,
{
    reader: &'a mut csv::Reader<R>,
}

impl<'a, R> TransactionParser<'a, R>
where
    R: std::io::Read,
{
    pub fn new(reader: &'a mut csv::Reader<R>) -> Self {
        Self { reader }
    }
}

impl<'a, R> Iterator for TransactionParser<'a, R>
where
    R: std::io::Read,
{
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader
            .deserialize()
            .next()
            .map(|result| result.expect("Incorrect CSV row."))
    }
}
