use serde::Serializer;

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
