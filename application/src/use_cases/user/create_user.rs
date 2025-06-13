use std::sync::Arc;

use domain::{
    entities::user::User,
    repositories::user_repository::{UserRepository, UserRepositoryError},
};

use crate::gateways::validations::email_validator::{self, EmailValidator};

#[derive(Debug)]
pub struct CreateUserInputData {
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub password_hash: String,
}

impl From<CreateUserInputData> for User {
    fn from(value: CreateUserInputData) -> Self {
        Self::new(
            value.username,
            value.email,
            value.avatar_url,
            value.password_hash,
        )
    }
}

#[derive(Debug)]
pub struct CreateUserResponse {
    pub new_id: u64,
}

#[derive(Debug)]
pub struct CreateUserUseCase<R: UserRepository, V1: EmailValidator> {
    user_repo: Arc<R>,
    email_validator: V1,
}

impl<R, V1> CreateUserUseCase<R, V1>
where
    R: UserRepository,
    V1: EmailValidator,
{
    pub fn new(user_repo: Arc<R>, email_validator: V1) -> Self {
        Self {
            user_repo,
            email_validator,
        }
    }
}

impl<R, V1> CreateUserUseCase<R, V1>
where
    R: UserRepository,
    V1: EmailValidator,
{
    pub async fn execute(
        &self,
        req: CreateUserInputData,
    ) -> Result<CreateUserResponse, CreateUserUseCaseError> {
        // verify email format here.
        if !self.email_validator.is_valid(&req.email) {
            return Err(CreateUserUseCaseError::InvalidEmailFormat);
        }

        // save user here.
        let new_id = self
            .user_repo
            .save(req.into())
            .await
            .map_err(|err| CreateUserUseCaseError::Repository(err.into()))?;

        Ok(CreateUserResponse { new_id })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserUseCaseError {
    #[error(transparent)]
    Repository(#[from] UserRepositoryError),

    #[error("The email address is already in use.")]
    EmailAlreadyInUse,
    #[error("The username is already in use.")]
    UsernameAlreadyInUse,
    #[error("Invalid email address format.")]
    InvalidEmailFormat,
}
