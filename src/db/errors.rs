use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum ItemType {
    Group,
    Expense,
    RepeatingExpense,
    ExpenseCategory,
    Budget,
    BudgetCategory,
    BudgetView,
    SavingGoal,
}

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("MongoDB error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("duplicate error: {0}")]
    MongoErrorKind(mongodb::error::ErrorKind),
    #[error("duplicate error: {0}")]
    MongoDuplicateError(String),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),
    #[error("validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("invalid ID: {0}")]
    InvalidIDError(String),
    #[error("{0:?} with ID: {1} not found")]
    NotFoundError(ItemType, String),
    #[error("Deserialization error")]
    DeserializationError(#[from] mongodb::bson::de::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl Into<(axum::http::StatusCode, Json<serde_json::Value>)> for DBError {
    fn into(self) -> (axum::http::StatusCode, Json<serde_json::Value>) {
        let (status, error_response) = match self {
            DBError::MongoErrorKind(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error kind: {}", e),
                },
            ),
            DBError::MongoDuplicateError(_) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail",
                    message: "Note with that title already exists".to_string(),
                },
            ),
            DBError::InvalidIDError(id) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: "fail",
                    message: format!("invalid ID: {}", id),
                },
            ),
            DBError::NotFoundError(item_type, id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail",
                    message: format!("{:?} with ID: {} not found", item_type, id),
                },
            ),
            DBError::MongoError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DBError::MongoQueryError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DBError::MongoSerializeBsonError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DBError::MongoDataError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DBError::DeserializationError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("Deserialization error: {}", e),
                },
            ),
        };
        (status, Json(serde_json::to_value(error_response).unwrap()))
    }
}

impl From<DBError> for (StatusCode, ErrorResponse) {
    fn from(err: DBError) -> (StatusCode, ErrorResponse) {
        err.into()
    }
}
