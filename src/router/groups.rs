use super::utils::{CreateDataResponse, HomeResponse, HomeStatusCode, UserInformation};
use crate::{
    db::m_group::{AssignMembers, CreateGroup, GroupStatus, GroupType, UpdateGroup},
    AppState,
};

use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct GroupList {
    pub id: String,
    pub name: String,
    pub status: GroupStatus,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
}

// TODO: Better Error handling with Status Codes
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
    // TO DO: Think more about the query parameters (Filters, Sorting, Pagination and Deleted groups)
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
        code: HomeStatusCode::Success,
        message: "List of groups".to_string(),
        data: Some(groups),
    })
}

#[axum::debug_handler]
async fn create_group(
    Extension(state): Extension<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateGroup>,
) -> Json<HomeResponse<CreateDataResponse>> {
    let db = &state.db;
    let group = payload;
    let id = db.create_group(group).await.unwrap();
    Json(HomeResponse {
        code: HomeStatusCode::Success,
        message: "Group created successfully".to_string(),
        data: Some(CreateDataResponse { id }),
    })
}

async fn get_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> impl IntoResponse {
    let db = &state.db;
    match db.get_group(group_id).await {
        Ok(group) => {
            return (
                StatusCode::OK,
                Json(HomeResponse {
                    code: HomeStatusCode::Success,
                    message: "Group details".to_string(),
                    data: Some(group),
                }),
            );
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HomeResponse {
                    code: HomeStatusCode::Error,
                    message: format!("Error getting group: {}", err),
                    data: None,
                }),
            );
        }
    }
}

async fn update_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(group_id): Path<String>,
    extract::Json(payload): extract::Json<UpdateGroup>,
) -> Json<HomeResponse<CreateDataResponse>> {
    let db = &state.db;
    let group = payload;
    match db.update_group(group_id.clone(), group).await {
        Ok(id) => Json(HomeResponse {
            code: HomeStatusCode::Success,
            message: "Group updated successfully".to_string(),
            data: Some(CreateDataResponse { id }),
        }),
        Err(err) => Json(HomeResponse {
            code: HomeStatusCode::Error,
            message: format!("Failed to update Group: {}", err),
            data: None,
        }),
    }
}

async fn delete_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> Json<HomeResponse<()>> {
    let db = &state.db;
    match db.disband_group(group_id).await {
        Ok(_) => Json(HomeResponse {
            code: HomeStatusCode::Success,
            message: "Group deleted successfully".to_string(),
            data: None,
        }),
        Err(err) => Json(HomeResponse {
            code: HomeStatusCode::Error,
            message: format!("Failed to delete Group: {}", err),
            data: None,
        }),
    }
}

async fn assign_members(
    Extension(state): Extension<Arc<AppState>>,
    Extension(user_info): Extension<UserInformation>,
    Path(group_id): Path<String>,
    extract::Json(payload): extract::Json<AssignMembers>,
) -> (StatusCode, Json<HomeResponse<()>>) {
    let db = &state.db;
    let user_id = user_info.user_id.clone();
    match db.is_user_admin(&group_id, &user_id).await {
        Ok(true) => {
            return match db.assign_members(group_id, &user_id, payload.members).await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(HomeResponse {
                        code: HomeStatusCode::Success,
                        message: "Members assigned successfully".to_string(),
                        data: None,
                    }),
                ),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(HomeResponse {
                        code: HomeStatusCode::Error,
                        message: format!("Failed to assign members: {}", err),
                        data: None,
                    }),
                ),
            }
        }
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(HomeResponse {
                code: HomeStatusCode::Unauthorized,
                message: "You are not an admin of this group".to_string(),
                data: None,
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HomeResponse {
                code: HomeStatusCode::Error,
                message: format!("Failed to check if user is admin: {}", err),
                data: None,
            }),
        ),
    }
}
