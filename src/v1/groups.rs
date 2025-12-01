use crate::v1::OpenCloudV1;

pub struct OcV1Groups<'c> {
    pub(crate) v1: &'c OpenCloudV1<'c>,
}
