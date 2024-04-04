use xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(source = "dawproject/Project.xsd")]
struct ProjectSchema;
