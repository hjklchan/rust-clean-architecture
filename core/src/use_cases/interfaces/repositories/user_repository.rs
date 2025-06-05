use crate::{
    entities::user::User, use_cases::interfaces::repositories::error::UserRepositoryError,
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<u64, UserRepositoryError>;
    async fn soft_delete(&self, id: u64) -> Result<User, UserRepositoryError>;
    async fn forced_delete(&self, id: u64) -> Result<User, UserRepositoryError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError>;
    async fn check_if_user_exist(&self, id: u64) -> Result<bool, UserRepositoryError>;
    async fn check_if_email_is_in_use(&self, email: &str) -> Result<bool, UserRepositoryError>;
}
