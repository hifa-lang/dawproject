const PROJECT_XSD: &str = include_str!("dawproject/project.xsd");

fn main() {
    let fixed_project_xsd = PROJECT_XSD.replace(
        "<xs:attribute name=\"contentType\">",
        "<xs:attribute name=\"contentTypes\">",
    );

    // Write file
    std::fs::write("assets/project.xsd", fixed_project_xsd).unwrap();
}