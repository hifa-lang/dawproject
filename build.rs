use std::process::Command;

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .output()
            .expect("Sync submodules failed!");

        // MetaData.xsd
        let fixed_metadata_xsd: String =
            std::fs::read_to_string("dawproject/MetaData.xsd").unwrap();

        // Project.xsd
        let project_xsd: String = std::fs::read_to_string("dawproject/Project.xsd").unwrap();
        let fixed_project_xsd = project_xsd.replace(
            "<xs:attribute name=\"contentType\">",
            "<xs:attribute name=\"contentTypes\">",
        );

        // Write file
        std::fs::write("assets/FixedMetaData.xsd", fixed_metadata_xsd).unwrap();
        std::fs::write("assets/FixedProject.xsd", fixed_project_xsd).unwrap();
    }
}
