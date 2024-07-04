use hifa_xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};

#[allow(dead_code)]
#[derive(Debug, XmlSchema)]
#[xml_schema(source = "assets/FixedProject.xsd")]
struct ProjectSchema;
