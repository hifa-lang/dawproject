use std::process::Command;

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .output()
            .expect("Sync submodules failed!");

        const PROJECT_XSD: &str = include_str!("./dawproject/Project.xsd");

        let fixed_project_xsd = PROJECT_XSD.replace(
            "<xs:attribute name=\"contentType\">",
            "<xs:attribute name=\"contentTypes\">",
        );

        // Write file
        std::fs::write("assets/Project.xsd", fixed_project_xsd).unwrap();
    }
}
