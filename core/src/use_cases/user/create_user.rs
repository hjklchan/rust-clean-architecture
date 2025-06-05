use thiserror::Error;

use crate::{
    entities::user::User,
    use_cases::interfaces::repositories::{UserRepository, error::UserRepositoryError},
};

pub struct CreateUserUseCase<R1>
where
    R1: UserRepository,
{
    user_repo: R1,
}

impl<R1> CreateUserUseCase<R1>
where
    R1: UserRepository,
{
    pub fn execute(&self, user: User) -> Result<u64, CreateUserError> {
        if self.user_repo.check_if_email_exist(&user.email) {
            return Err(CreateUserError::EmailAlreadyInUse);
        }

        let id = self
            .user_repo
            .save(user)
            .map_err(|err| CreateUserError::from(err))?;

        Ok(id)
    }
}

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("{}", UserRepositoryError::DatasourceAccessError)]
    RepositoryError,
    #[error("The e-mail address has already been used")]
    EmailAlreadyInUse,
    #[error("{}", UserRepositoryError::UserAlreadyExists)]
    UserAlreadyExists,
    #[error("{}", UserRepositoryError::UserNotFound)]
    UserNotFound,
}

impl From<UserRepositoryError> for CreateUserError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::DatasourceAccessError => CreateUserError::RepositoryError,
            UserRepositoryError::EmailAlreadyInUse => CreateUserError::EmailAlreadyInUse,
            UserRepositoryError::UserAlreadyExists => CreateUserError::UserAlreadyExists,
            UserRepositoryError::UserNotFound => CreateUserError::UserNotFound,
        }
    }
}
