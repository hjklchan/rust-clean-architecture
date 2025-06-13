use std::sync::Arc;

use chrono::{DateTime, Utc};
use domain::{
    entities::user::{User, UserStatus as DomainUserStatus},
    repositories::user_repository::{UserRepository, UserRepositoryError},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UserStatus {
    Normal,
    Frozen,
    Disabled,
}

impl From<DomainUserStatus> for UserStatus {
    fn from(value: DomainUserStatus) -> Self {
        match value {
            DomainUserStatus::Normal => Self::Normal,
            DomainUserStatus::Frozen => Self::Frozen,
            DomainUserStatus::Disabled => Self::Disabled,
        }
    }
}

#[derive(Debug)]
pub struct GetUserOutputData {
    pub id: Option<u64>,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct GetUserByIdUseCase<R: UserRepository> {
    user_repo: Arc<R>,
}

impl<R> GetUserByIdUseCase<R>
where R: UserRepository {
    pub fn new(user_repo: Arc<R>) -> Self {
        Self { user_repo }
    }
}

impl<R> GetUserByIdUseCase<R>
where
    R: UserRepository,
{
    pub async fn execute(&self, id: u64) -> Result<GetUserOutputData, GetUserByIdUseCaseError> {
        let user = self
            .user_repo
            .get_by_id(id)
            .await
            .map_err(|err| GetUserByIdUseCaseError::Repository(err.into()))?
            .ok_or(GetUserByIdUseCaseError::UserNotFound)?;

        Ok(GetUserOutputData {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            status: user.status.into(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetUserByIdUseCaseError {
    #[error(transparent)]
    Repository(#[from] UserRepositoryError),

    #[error("User not found.")]
    UserNotFound,
}
