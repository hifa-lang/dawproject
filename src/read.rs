use crate::utils::consts::{METADATA_PATH, PROJECT_PATH};
use crate::{Dawproject, MetaData, Project};
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

// TODO: Read other files in `.dawproject` file.
/// Read `.dawproject` file.
#[derive(Clone, Debug)]
pub struct DawprojectReader<R> {
    zip: zip::ZipArchive<R>,
    file_names: Vec<String>,
    metadata: Option<MetaData>,
    project: Option<Project>,
}

impl<R> DawprojectReader<R>
where
    R: Read + Seek,
{
    pub fn new(reader: R) -> Result<Self, DawprojectReadError> {
        let zip = zip::ZipArchive::new(reader)?;
        let mut file_names = zip
            .file_names()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        // sort the file names for consistent testing.
        file_names.sort();

        Ok(DawprojectReader {
            zip,
            file_names,
            metadata: None,
            project: None,
        })
    }

    // TODO: check if the file exists.
    // TODO: check if file is already read.
    fn read_metadata(&mut self) -> Result<(), DawprojectReadError> {
        let metadata_xml = self.zip.by_name(METADATA_PATH)?;
        let metadata: MetaData = yaserde::de::from_reader(metadata_xml)
            .map_err(DawprojectReadError::MetadataDeserializeError)?;
        self.metadata = Some(metadata);
        Ok(())
    }
    fn read_project(&mut self) -> Result<(), DawprojectReadError> {
        let project_xml = self.zip.by_name(PROJECT_PATH)?;
        let project: Project = yaserde::de::from_reader(project_xml)
            .map_err(DawprojectReadError::ProjectDeserializeError)?;
        self.project = Some(project);
        Ok(())
    }

    /// Read metadata and project.
    pub fn read_dawproject(&mut self) -> Result<(), DawprojectReadError> {
        self.read_metadata()?;
        self.read_project()?;
        Ok(())
    }

    pub fn file_names(&self) -> impl Iterator<Item = &str> {
        self.file_names.iter().map(|s| s.as_str())
    }

    /// Build a `Dawproject` structure.
    /// If metadata and project are not read, return `None`.
    pub fn build_dawproject(&mut self) -> Option<Dawproject> {
        if self.metadata.is_some() && self.project.is_some() {
            Some(Dawproject::new(
                self.metadata.take().unwrap(),
                self.project.take().unwrap(),
            ))
        } else {
            None
        }
    }

    pub fn by_name(&mut self, name: &str) -> Result<zip::read::ZipFile, DawprojectReadError> {
        self.zip
            .by_name(name)
            .map_err(DawprojectReadError::ZipError)
    }

    // Extract `.dawproject` file to a directory.
    pub fn extract<P: AsRef<Path>>(&mut self, directory: P) -> Result<(), DawprojectReadError> {
        self.zip
            .extract(directory)
            .map_err(DawprojectReadError::ZipError)
    }
}

impl DawprojectReader<BufReader<File>> {
    /// Open a `.dawproject` file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DawprojectReadError> {
        let f = File::open(path).map_err(DawprojectReadError::StdIoError)?;
        let reader = BufReader::new(f);
        Self::new(reader)
    }
}
