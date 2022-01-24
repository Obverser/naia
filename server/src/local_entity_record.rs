use naia_shared::LocalEntity;

use super::locality_status::LocalityStatus;

#[derive(Debug)]
pub struct LocalEntityRecord {
    pub local_key: LocalEntity,
    pub status: LocalityStatus,
}

impl LocalEntityRecord {
    pub fn new(local_key: LocalEntity) -> Self {
        LocalEntityRecord {
            local_key,
            status: LocalityStatus::Creating,
        }
    }
}
