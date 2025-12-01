use crate::{
    Error,
    v2::{OpenCloudV2, Pager, PagerOptions},
};
use serde::Deserialize;

const BASE_API: &str = "https://apis.roblox.com";

pub struct OcV2Groups<'c> {
    pub(crate) v2: &'c OpenCloudV2<'c>,
}

impl<'c> OcV2Groups<'c> {
    pub async fn group_info(&self, group_id: u64) -> Result<GroupInfo, Error> {
        Ok(self
            .v2
            .get(format!("{BASE_API}/cloud/v2/groups/{group_id}"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub fn list_roles(&self, group_id: u64, options: PagerOptions) -> Pager<'c, GroupRole> {
        Pager::new(
            "groupRoles".into(),
            format!("{BASE_API}/cloud/v2/groups/{group_id}/roles?"),
            &self.v2,
            options,
        )
    }

    pub fn list_memberships(
        &self,
        group_id: u64,
        options: PagerOptions,
    ) -> Pager<'c, GroupMembership> {
        Pager::new(
            "groupMemberships".into(),
            format!("{BASE_API}/cloud/v2/groups/{group_id}/memberships?"),
            &self.v2,
            options,
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub path: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub rank: u8,
    pub member_count: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub user: String,
    pub role: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub owner: String,
    pub member_count: u64,
    pub public_entry_allowed: bool,
    pub locked: bool,
    pub verified: bool,
}
