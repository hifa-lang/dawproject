use dawproject::{DawprojectReader, DawprojectWriter};
use std::{fs::File, io::Read};
use zip::ZipArchive;

#[test]
fn io_canon_dawproject() {
    // Read
    let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();
    reader.read_dawproject().unwrap();
    let dawproject = reader.build_dawproject().unwrap();
    // Write
    let mut writer = DawprojectWriter::create("tests/data/copied_canon.dawproject").unwrap();
    writer.write_dawproject(&dawproject).unwrap();

    // IO other files
    let files: Vec<String> = reader.file_names().map(String::from).collect();
    for file in files {
        if file != "metadata.xml" && file != "project.xml" {
            let file = reader.by_name(&file).unwrap();
            writer.raw_copy_file(file).unwrap();
        }
    }
    writer.finish().unwrap();

    // Check other files
    let mut original = ZipArchive::new(File::open("tests/data/canon.dawproject").unwrap()).unwrap();
    let mut copied =
        ZipArchive::new(File::open("tests/data/copied_canon.dawproject").unwrap()).unwrap();
    for i in 0..original.len() {
        let mut original_file = original.by_index(i).unwrap();
        let mut copied_file = copied.by_name(original_file.name()).unwrap();
        assert_eq!(original_file.name(), copied_file.name());
        if matches!(original_file.name(), "metadata.xml" | "project.xml") {
            continue;
        }
        let mut original_buf = Vec::new();
        let mut copied_buf = Vec::new();
        original_file.read_to_end(&mut original_buf).unwrap();
        copied_file.read_to_end(&mut copied_buf).unwrap();
        assert_eq!(original_buf, copied_buf);
    }

    // Check metadata and project
    let mut copied_reader = DawprojectReader::open("tests/data/copied_canon.dawproject").unwrap();
    copied_reader.read_dawproject().unwrap();
    let copied_dawproject = copied_reader.build_dawproject().unwrap();
    assert_eq!(dawproject, copied_dawproject);
}
