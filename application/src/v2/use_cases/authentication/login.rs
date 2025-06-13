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
pub enum LoginUseCaseError {}
