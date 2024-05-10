use hifa_xml_schema_derive::XmlSchema;

#[derive(Debug, XmlSchema)]
#[xml_schema(source = "assets/FixedMetaData.xsd")]
struct MetadataSchema;
