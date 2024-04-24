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
