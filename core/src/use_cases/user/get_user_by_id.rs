use thiserror::Error;

use crate::{
    entities::user::User,
    use_cases::interfaces::repositories::{UserRepository, error::UserRepositoryError},
};

pub struct GetUserByIdUseCase<R1>
where
    R1: UserRepository,
{
    user_repo: R1,
}

impl<R1> GetUserByIdUseCase<R1>
where
    R1: UserRepository,
{
    pub fn execute(&self, id: u64) -> Result<User, GetUserByIdError> {
        let user = self
            .user_repo
            .get_by_id(id)
            .map_err(|err| GetUserByIdError::from(err))?;

        Ok(user)
    }
}

#[derive(Debug, Error)]
pub enum GetUserByIdError {
    #[error("{}", UserRepositoryError::DatasourceAccessError)]
    RepositoryError,
    #[error("user not found")]
    UserNotFound,
}

impl From<UserRepositoryError> for GetUserByIdError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::UserNotFound => Self::UserNotFound,
            _ => Self::RepositoryError,
        }
    }
}
