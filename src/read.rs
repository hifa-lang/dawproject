use crate::{MetaData, Project};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DawprojectReadError {
    #[error("zip extraction error")]
    ZipError(#[from] zip::result::ZipError),
    #[error("metadata.xml deserialize error: {0}")]
    MetadataDeserializeError(String),
    #[error("project.xml deserialize error: {0}")]
    ProjectDeserializeError(String),
    #[error("std io error")]
    StdIoError(#[from] std::io::Error),
}

/// Read the `.dawproject` file.
/// TODO: Read other files in the `.dawproject` file.
pub struct DawprojectReader<R> {
    zip: zip::ZipArchive<R>,
    metadata: Option<MetaData>,
    project: Option<Project>,
}

impl<R> DawprojectReader<R>
where
    R: Read + Seek,
{
    pub fn new(reader: R) -> Result<Self, DawprojectReadError> {
        let zip = zip::ZipArchive::new(reader)?;

        Ok(DawprojectReader {
            zip,
            metadata: None,
            project: None,
        })
    }

    pub fn read_metadata(&mut self) -> Result<(), DawprojectReadError> {
        let metadata_xml = self.zip.by_name("metadata.xml")?;
        let metadata: MetaData = yaserde::de::from_reader(metadata_xml)
            .map_err(DawprojectReadError::MetadataDeserializeError)?;
        self.metadata = Some(metadata);
        Ok(())
    }
    pub fn read_project(&mut self) -> Result<(), DawprojectReadError> {
        let project_xml = self.zip.by_name("project.xml")?;
        let project: Project = yaserde::de::from_reader(project_xml)
            .map_err(DawprojectReadError::ProjectDeserializeError)?;
        self.project = Some(project);
        Ok(())
    }

    pub fn metadata(&self) -> Option<&MetaData> {
        self.metadata.as_ref()
    }
    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }
}

impl DawprojectReader<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DawprojectReadError> {
        let f = File::open(path).map_err(DawprojectReadError::StdIoError)?;
        let reader = BufReader::new(f);
        Self::new(reader)
    }
}
