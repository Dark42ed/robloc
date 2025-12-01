use serde::Deserialize;

use crate::{
    Error,
    v1::UniverseId,
    v2::{OpenCloudV2, Pager, PagerOptions},
};

const BASE_API: &str = "https://apis.roblox.com";

pub struct OcV2Datastore<'c> {
    pub(crate) v2: &'c OpenCloudV2<'c>,
}

impl<'c> OcV2Datastore<'c> {
    pub fn list_datastores(
        &self,
        universe_id: UniverseId,
        options: PagerOptions,
    ) -> Pager<'c, Datastore> {
        Pager::new(
            "dataStores".into(),
            format!("{BASE_API}/cloud/v2/universes/{universe_id}/data-stores"),
            self.v2,
            options,
        )
    }

    pub async fn get_entry(
        &self,
        universe_id: UniverseId,
        datastore_id: &str,
        entry_id: &str,
    ) -> Result<DatastoreEntry, Error> {
        self.v2.get_default(&format!("{BASE_API}/cloud/v2/universes/{universe_id}/data-stores/{datastore_id}/entries/{entry_id}")).await
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datastore {
    pub path: String,
    pub create_time: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatastoreEntry {
    pub path: String,
    pub create_time: String,
    pub revision_id: String,
    pub revision_create_time: String,
    pub state: String,
    pub etag: String,
    pub value: serde_json::Value,
    pub id: String,
    #[serde(default)]
    pub users: Vec<String>,
    pub attributes: serde_json::Value,
}
