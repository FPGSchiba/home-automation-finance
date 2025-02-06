use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum StatusCode {
    Success,
    Unauthorized,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct HomeResponse<T: Serialize> {
    pub code: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
