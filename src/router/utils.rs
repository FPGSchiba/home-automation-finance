use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum HomeStatusCode {
    Success,
    Unauthorized,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct HomeResponse<T: Serialize> {
    pub code: HomeStatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDataResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInformation {
    pub user_id: String,
}
