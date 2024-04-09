use crate::{MetaData, Project};

#[derive(Clone, Debug, PartialEq)]
pub struct Dawproject {
    pub metadata: MetaData,
    pub project: Project,
}

impl Dawproject {
    pub fn new(metadata: MetaData, project: Project) -> Self {
        Dawproject { metadata, project }
    }
}
