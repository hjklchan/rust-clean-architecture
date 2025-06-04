use crate::{entities::user::User, use_cases::interfaces::repositories::UserRepository};

pub struct GetUserByIdUseCase<R1>
where
    R1: UserRepository,
{
    user_repo: R1,
}

impl<R1> GetUserByIdUseCase<R1>
where
    R1: UserRepository,
{
    pub fn execute(&self, id: u64) -> User {
        self.user_repo.get_by_id(id)
    }
}
