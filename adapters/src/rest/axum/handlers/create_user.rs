use std::sync::Arc;

use application::{
    gateways::validations::email_validator::EmailValidator,
    use_cases::user::create_user::{CreateUserInputData, CreateUserUseCase},
};
use axum::{Json, extract::State};
use domain::repositories::user_repository::UserRepository;
use serde::{Deserialize, Serialize};

use crate::{external::validations::email_validator_impl::EmailValidatorImpl, rest::axum::shared::{errors::AppError, states::UserAppState}};

#[derive(Debug, Deserialize)]
pub struct CreateUserApiRequest {
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub password_hash: String,
}

impl From<CreateUserApiRequest> for CreateUserInputData {
    fn from(value: CreateUserApiRequest) -> Self {
        Self {
            username: value.username,
            email: value.email,
            avatar_url: value.avatar_url,
            password_hash: value.password_hash,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateUserApiResponse {
    pub new_id: u64,
}

pub async fn create_user_handler<R: UserRepository>(
    state: State<UserAppState<R>>,
    req: Json<CreateUserApiRequest>,
) -> Result<Json<CreateUserApiResponse>, AppError> {
    let email_validator_impl = EmailValidatorImpl();

    let create_user_uc = CreateUserUseCase::new(Arc::clone(&state.user_repo), email_validator_impl)
        .execute(req.0.into())
        .await
        .map_err(|err| AppError::CreateUserUseCase(err.into()))?;

    Ok(Json::from(CreateUserApiResponse {
        new_id: create_user_uc.new_id,
    }))
}

