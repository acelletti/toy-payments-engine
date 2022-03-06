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
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}
