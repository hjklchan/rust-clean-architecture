use application::use_cases::user::{
    create_user::CreateUserUseCaseError, get_user_by_id::GetUserByIdUseCaseError,
};
use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};

use crate::rest::axum::shared::api_response::ApiErrorResponse;

pub enum AppError {
    JsonRejection(JsonRejection),
    CreateUserUseCase(CreateUserUseCaseError),
    GetUserByIdUseCase(GetUserByIdUseCaseError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Self::CreateUserUseCase(err) => match err {
                CreateUserUseCaseError::EmailAlreadyInUse => {
                    (StatusCode::NOT_FOUND, err.to_string())
                }
                CreateUserUseCaseError::UsernameAlreadyInUse => {
                    (StatusCode::NOT_FOUND, err.to_string())
                }
                CreateUserUseCaseError::Repository(repo_error) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, repo_error.to_string())
                }
                CreateUserUseCaseError::InvalidEmailFormat => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
            },
            Self::GetUserByIdUseCase(err) => match err {
                GetUserByIdUseCaseError::Repository(repo_err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, repo_err.to_string())
                }
                GetUserByIdUseCaseError::UserNotFound => (StatusCode::NOT_FOUND, err.to_string()),
            },
        };

        (status, Json(ApiErrorResponse::new("todo", message, None))).into_response()
    }
}
