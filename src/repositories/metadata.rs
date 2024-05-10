use hifa_xml_schema_derive::XmlSchema;

#[derive(Debug, XmlSchema)]
#[xml_schema(
    source = "https://raw.githubusercontent.com/hifa-lang/dawproject/main/assets/FixedMetaData.xsd"
)]
struct MetadataSchema;
