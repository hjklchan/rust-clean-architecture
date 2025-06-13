use std::sync::Arc;

use crate::rest::axum::shared::{errors::AppError, states::UserAppState};
use application::use_cases::user::get_user_by_id::{
    GetUserByIdUseCase, GetUserOutputData, UserStatus,
};
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use domain::repositories::user_repository::UserRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetUserByIdApiRequest {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub struct GetUserByIdApiResponse {
    pub id: Option<u64>,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GetUserOutputData> for GetUserByIdApiResponse {
    fn from(value: GetUserOutputData) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            avatar_url: value.avatar_url,
            status: match value.status {
                UserStatus::Normal => 0,
                UserStatus::Frozen => 1,
                UserStatus::Disabled => 2,
            },
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn get_user_by_id_handler<R: UserRepository>(
    state: State<UserAppState<R>>,
    req: Json<GetUserByIdApiRequest>,
) -> Result<Json<GetUserByIdApiResponse>, AppError> {
    let use_case_resp = GetUserByIdUseCase::new(Arc::clone(&state.user_repo))
        .execute(req.id)
        .await
        .map_err(|err| AppError::GetUserByIdUseCase(err))?;

    Ok(Json(use_case_resp.into()))
}
