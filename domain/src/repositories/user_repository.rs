use std::error::Error;
use uuid::Uuid;

use crate::v2::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<Uuid, UserRepositoryError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn soft_delete(&self, id: u64) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("An error occurred while accessing the database.")]
    DatabaseAccessError(#[from] Box<dyn Error>),
}
