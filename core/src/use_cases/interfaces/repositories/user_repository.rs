use crate::{
    entities::user::User, use_cases::interfaces::repositories::error::UserRepositoryError,
};

pub trait UserRepository {
    fn save(&self, user: User) -> Result<u64, UserRepositoryError>;
    fn soft_delete(&self, id: u64) -> Result<User, UserRepositoryError>;
    fn forced_delete(&self, id: u64) -> Result<User, UserRepositoryError>;
    fn get_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError>;
    fn check_if_user_exist(&self, id: u64) -> Result<bool, UserRepositoryError>;
    fn check_if_email_is_in_use(&self, email: &str) -> Result<bool, UserRepositoryError>;
}
