use dawproject::DawprojectReader;
use std::fs::File;

#[test]
fn read_canon_dawproject() {
    let file = File::open("tests/data/canon.dawproject").unwrap();
    let mut reader = DawprojectReader::new(file).unwrap();
    reader.read_metadata().unwrap();
    reader.read_project().unwrap();
}
