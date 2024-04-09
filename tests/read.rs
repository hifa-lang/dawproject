use dawproject::DawprojectReader;

#[test]
fn read_canon_dawproject() {
    let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();

    reader.read_dawproject().unwrap();
    let dawproject = reader.build_dawproject().unwrap();

    assert_eq!(dawproject.metadata.content.title, Some("Canon".into()));
    assert_eq!(dawproject.project.content.version, "1.0");
    assert_eq!(
        reader.file_names().collect::<Vec<&str>>(),
        [
            "audio/canon_guitar_150bpm.wav",
            "metadata.xml",
            "project.xml"
        ]
    );
}
