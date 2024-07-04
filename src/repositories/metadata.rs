use hifa_xml_schema_derive::XmlSchema;

#[allow(dead_code)]
#[derive(Debug, XmlSchema)]
#[xml_schema(source = "assets/FixedMetaData.xsd")]
struct MetadataSchema;
