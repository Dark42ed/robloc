use crate::{
    OpenCloud,
    v1::{datastore::OcV1Datastore, groups::OcV1Groups},
};

pub mod datastore;
pub mod groups;

pub type UniverseId = u64;

pub struct OpenCloudV1<'c> {
    pub(crate) oc: &'c OpenCloud,
}

impl<'c> OpenCloudV1<'c> {
    pub fn datastore(&'c self) -> OcV1Datastore<'c> {
        OcV1Datastore { v1: self }
    }

    pub fn groups(&'c self) -> OcV1Groups<'c> {
        OcV1Groups { v1: self }
    }
}
