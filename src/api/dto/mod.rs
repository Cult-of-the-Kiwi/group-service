use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::dto::GroupInfo;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub member_ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub group_id: Uuid,
}

pub type ListUserGroupsResponse = Vec<GroupInfo>;

#[derive(Serialize, Deserialize)]
pub struct GetGroupRequest {
    pub group_id: Uuid,
}
pub type GetGroupResponse = GroupInfo;

#[derive(Serialize, Deserialize)]
pub struct DeleteGroupRequest {
    pub group_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ListGroupMembersRequest {
    pub group_id: Uuid,
}

pub type ListGroupMembersResponse = Vec<Uuid>;

#[derive(Deserialize)]
pub struct AddGroupMembersRequest {
    pub group_id: Uuid,
    pub member_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct RemoveGroupMembersRequest {
    pub group_id: Uuid,
    pub member_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct GetGroupByChannelRequest {
    pub channel_id: Uuid,
}

pub type GetGroupByChannelResponse = GroupInfo;
