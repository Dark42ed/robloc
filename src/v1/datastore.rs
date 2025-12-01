use serde_json::Value as JsonValue;

use crate::{
    Error, get_header_str, parse_header,
    v1::{OpenCloudV1, UniverseId},
};

const BASE_API: &str = "https://apis.roblox.com/datastores";

#[derive(Debug)]
pub struct DatastoreEntry {
    pub content: String,
    pub content_md5: String,
    pub entry_version: String,
    pub entry_created_time: String,
    pub entry_version_created_time: String,
    pub entry_attributes: serde_json::Map<String, JsonValue>,
    pub entry_userids: Vec<u64>,
}

pub struct OcV1Datastore<'c> {
    pub(crate) v1: &'c OpenCloudV1<'c>,
}

impl<'c> OcV1Datastore<'c> {
    pub async fn get_entry(
        &self,
        universe_id: UniverseId,
        datastore_name: &str,
        entry_key: &str,
        scope: Option<&str>,
    ) -> Result<DatastoreEntry, Error> {
        let mut url = format!(
            "{BASE_API}/v1/universes/{universe_id}/standard-datastores/datastore/entries/entry?datastoreName={datastore_name}&entryKey={entry_key}"
        );
        if let Some(scope) = scope {
            url += "&scope=";
            url += scope;
        }

        let request = self
            .v1
            .oc
            .client
            .get(url)
            .header("x-api-key", &*self.v1.oc.secret)
            .send()
            .await?
            .error_for_status()?;

        let headers = request.headers();

        Ok(DatastoreEntry {
            content_md5: get_header_str(headers, "content-md5")?.unwrap().to_string(),
            entry_version: get_header_str(headers, "roblox-entry-version")?
                .unwrap()
                .to_string(),
            entry_created_time: get_header_str(headers, "roblox-entry-created-time")?
                .unwrap()
                .to_string(),
            entry_version_created_time: get_header_str(
                headers,
                "roblox-entry-version-created-time",
            )?
            .unwrap()
            .to_string(),
            entry_attributes: parse_header::<serde_json::Map<String, JsonValue>, _>(
                headers,
                "roblox-entry-attributes",
            )?
            .unwrap_or_default(),
            entry_userids: parse_header::<Vec<u64>, _>(headers, "roblox-entry-userids")?
                .unwrap_or_default(),
            content: request.text().await?,
        })
    }
}
