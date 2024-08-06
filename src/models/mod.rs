pub mod bases;
pub mod parameters;

use zip::ZipArchive;

use crate::{MetaData, Project};

/// Dawproject structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dawproject {
    pub metadata: MetaData,
    pub project: Project,
}

impl Dawproject {
    pub fn new(metadata: MetaData, project: Project) -> Self {
        Dawproject { metadata, project }
    }

    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
}

pub struct DawprojectWithZip<R> {
    pub dawproject: Dawproject,
    pub zip: ZipArchive<R>,
}

impl<R> DawprojectWithZip<R> {
    pub fn new(dawproject: Dawproject, zip: ZipArchive<R>) -> Self {
        DawprojectWithZip { dawproject, zip }
    }

    pub fn metadata(&self) -> &MetaData {
        &self.dawproject.metadata
    }

    pub fn project(&self) -> &Project {
        &self.dawproject.project
    }
}
