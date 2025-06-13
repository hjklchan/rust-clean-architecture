use thiserror::Error;

pub struct RegisterInputData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterOutputData {
    pub id: String,
    pub token: String,
}

#[async_trait::async_trait]
pub trait RegisterUseCase {
    async fn execute(&self, input: RegisterInputData) -> Result<RegisterOutputData, RegisterUseCaseError>;
}

#[derive(Debug, Error)]
pub enum RegisterUseCaseError {}
