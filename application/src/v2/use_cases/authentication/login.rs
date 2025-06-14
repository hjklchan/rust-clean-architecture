use domain::repositories::user_repository::UserRepositoryError;
use thiserror::Error;

pub struct LoginInputData {
    pub email: String,
    pub password: String,
}

pub struct LoginOutputData {
    pub id: String,
    pub token: String,
}

#[async_trait::async_trait]
pub trait LoginUseCase {
    async fn execute(&self, input: LoginInputData) -> Result<LoginOutputData, LoginUseCaseError>;
}

#[derive(Debug, Error)]
pub enum LoginUseCaseError {
    #[error(transparent)]
    Repository(#[from] UserRepositoryError),

    #[error("User not found.")]
    UserNotFound,
    #[error("Incorrect password.")]
    IncorrectPassword,
    #[error("{0}")]
    JwtGenerateError(Box<dyn std::error::Error>),
    #[error("{0}")]
    ParsePasswordError(Box<dyn std::error::Error>),
}
