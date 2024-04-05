use dawproject::DawprojectReader;

#[test]
fn read_canon_dawproject() {
    let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();

    reader.read_metadata().unwrap();
    reader.read_project().unwrap();
}
