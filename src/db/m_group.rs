use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum GroupStatus {
    Active,
    Disbanded,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GroupType {
    Personal,
    Combined,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub admins: Vec<ObjectId>,
    pub members: Vec<ObjectId>,
    pub status: GroupStatus,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub disbanded_at: Option<DateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ListingGroup {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub status: GroupStatus,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
}
