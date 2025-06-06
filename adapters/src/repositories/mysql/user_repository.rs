use internal_core::{
    entities::user::User,
    use_cases::gateways::repositories::{UserRepository, error::UserRepositoryError},
};

use async_trait::async_trait;

pub struct MysqlUserRepository {}

#[async_trait]
impl UserRepository for MysqlUserRepository {
    async fn save(&self, user: User) -> Result<u64, UserRepositoryError> {
        todo!()
    }

    async fn soft_delete(&self, id: u64) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn forced_delete(&self, id: u64) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError> {
        todo!()
    }

    async fn check_if_user_exist(&self, id: u64) -> Result<bool, UserRepositoryError> {
        todo!()
    }

    async fn check_if_email_is_in_use(&self, email: &str) -> Result<bool, UserRepositoryError> {
        todo!()
    }
}
