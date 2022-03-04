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
