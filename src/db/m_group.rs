use crate::db::utils::{
    deserialize_bson_datetime_from_rfc3339_string,
    deserialize_option_bson_datetime_from_rfc3339_string,
    deserialize_vec_object_id_from_hex_string, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_vec_object_id_as_hex_string,
};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum GroupStatus {
    Active,
    Disbanded,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GroupType {
    Personal,
    Combined,
}

#[derive(Serialize, Deserialize)]
pub enum GroupRole {
    Admin,
    Member,
}

#[derive(Serialize, Deserialize)]
pub struct BsonGroup {
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
    pub updated_at: Option<DateTime>,
    #[serde(rename = "disbandedAt")]
    pub disbanded_at: Option<DateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonGroup {
    pub id: String,
    pub name: String,
    pub admins: Vec<String>,
    pub members: Vec<String>,
    pub status: GroupStatus,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "disbandedAt")]
    pub disbanded_at: Option<String>,
}

impl From<BsonGroup> for JsonGroup {
    fn from(bson_group: BsonGroup) -> Self {
        JsonGroup {
            id: bson_group
                .id
                .map_or_else(|| "".to_string(), |id| id.to_hex()),
            name: bson_group.name,
            admins: bson_group
                .admins
                .into_iter()
                .map(|id| id.to_hex())
                .collect(),
            members: bson_group
                .members
                .into_iter()
                .map(|id| id.to_hex())
                .collect(),
            status: bson_group.status,
            group_type: bson_group.group_type,
            created_at: bson_group
                .created_at
                .try_to_rfc3339_string()
                .unwrap_or_else(|_| "".to_string()),
            updated_at: bson_group.updated_at.map(|dt| {
                dt.try_to_rfc3339_string()
                    .unwrap_or_else(|_| "".to_string())
            }),
            disbanded_at: bson_group.disbanded_at.map(|dt| {
                dt.try_to_rfc3339_string()
                    .unwrap_or_else(|_| "".to_string())
            }),
        }
    }
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

#[derive(Serialize, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub admins: Vec<String>,
    pub members: Vec<String>,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateGroup {
    pub name: Option<String>,
    #[serde(rename = "groupType")]
    pub group_type: Option<GroupType>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupMembership {
    pub group_id: ObjectId,
    pub role: GroupRole,
}
