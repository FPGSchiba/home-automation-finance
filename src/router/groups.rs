use super::utils::{HomeResponse, StatusCode};
use crate::{
    db::m_group::{Group, GroupStatus, GroupType, ListingGroup},
    AppState,
};

use axum::{
    extract::{self, Path},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct CreateGroup {
    name: String,
    admins: Vec<ObjectId>,
    members: Vec<ObjectId>,
    #[serde(rename = "groupType")]
    group_type: GroupType,
}

#[derive(Serialize, Deserialize)]
struct GroupList {
    pub id: String,
    pub name: String,
    pub status: GroupStatus,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
}

pub fn get_group_router() -> Router {
    Router::new()
        .route("/", get(list_groups))
        .route("/", post(create_group))
        .route("/{group_id}", get(get_group))
        .route("/{group_id}", put(update_group))
        .route("/{group_id}", delete(delete_group))
        .route("/{group_id}", post(assign_members))
}

async fn list_groups(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<HomeResponse<Vec<GroupList>>> {
    let db = &state.db;
    let groups = db
        .list_groups()
        .await
        .unwrap()
        .iter()
        .map(|group| GroupList {
            id: group.id.as_ref().unwrap().to_hex(),
            name: group.name.clone(),
            status: group.status.clone(),
            group_type: group.group_type.clone(),
        })
        .collect();
    Json(HomeResponse {
        code: StatusCode::Success,
        message: "List of groups".to_string(),
        data: Some(groups),
    })
}

#[axum::debug_handler]
async fn create_group(
    Extension(state): Extension<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateGroup>,
) -> Json<HomeResponse<()>> {
    let db = &state.db;
    let group = payload;
    db.insert_one(Group {
        id: None,
        name: group.name,
        admins: group.admins,
        members: group.members,
        status: GroupStatus::Active,
        group_type: group.group_type,
        created_at: chrono::Utc::now().into(),
        disbanded_at: None,
    })
    .await
    .unwrap();
    Json(HomeResponse {
        code: StatusCode::Success,
        message: "Group created successfully".to_string(),
        data: None,
    })
}

async fn get_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn update_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn delete_group(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}

async fn assign_members(Path(group_id): Path<String>) -> &'static str {
    "not implemented"
}
