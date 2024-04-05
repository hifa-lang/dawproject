use dawproject::DawprojectReader;

#[test]
fn read_canon_dawproject() {
    let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();

    reader.read_metadata().unwrap();
    reader.read_project().unwrap();

    let metadata = reader.metadata().unwrap();
    let project = reader.project().unwrap();

    assert_eq!(metadata.content.title, Some("Canon".into()));
    assert_eq!(project.content.version, "1.0");
    assert_eq!(
        reader.file_names().collect::<Vec<&str>>(),
        [
            "audio/canon_guitar_150bpm.wav",
            "metadata.xml",
            "project.xml"
        ]
    );
}
