use thiserror::Error;

use crate::{
    entities::user::User,
    use_cases::interfaces::repositories::{UserRepository, error::UserRepositoryError},
};

pub struct SaveUserUseCase<R1>
where
    R1: UserRepository,
{
    user_repo: R1,
}

impl<R1> SaveUserUseCase<R1>
where
    R1: UserRepository,
{
    pub fn execute(&self, user: User) -> Result<u64, SaveUserError> {
        let id = self
            .user_repo
            .save(user)
            .map_err(|err| SaveUserError::from(err))?;

        Ok(id)
    }
}

#[derive(Debug, Error)]
pub enum SaveUserError {
    #[error("{}", UserRepositoryError::DatasourceAccessError)]
    RepositoryError,
    #[error("{}", UserRepositoryError::EmailAlreadyInUse)]
    EmailAlreadyInUseError,
}

impl From<UserRepositoryError> for SaveUserError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::EmailAlreadyInUse => Self::EmailAlreadyInUseError,
            _ => Self::RepositoryError,
        }
    }
}
