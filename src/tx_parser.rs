/// CSV parser wrapper that processes CSV data into Transaction data structure

#[cfg(test)]
mod tests;

use crate::models::transaction::Transaction;

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
