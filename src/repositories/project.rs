use hifa_xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(source = "assets/Project.xsd")]
struct ProjectSchema;
