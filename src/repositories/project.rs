use hifa_xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(
    source = "https://raw.githubusercontent.com/hifa-lang/dawproject/main/assets/FixedProject.xsd"
)]
struct ProjectSchema;
