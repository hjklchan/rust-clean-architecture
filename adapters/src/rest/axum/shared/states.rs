use std::sync::Arc;

use domain::repositories::user_repository::UserRepository;

#[derive(Debug, Clone)]
pub struct UserAppState<R: UserRepository> {
    pub user_repo: Arc<R>,
}
