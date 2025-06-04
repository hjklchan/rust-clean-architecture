use crate::{entities::user::User, use_cases::interfaces::repositories::UserRepository};

pub struct SaveUserUseCase<R1>
where
    R1: UserRepository,
{
    user_repo: R1,
}

impl<R1> SaveUserUseCase<R1>
where
    R1: UserRepository,
{
    pub fn execute(&self, user: User) -> u64 {
        let id = self.user_repo.save(user);
        id
    }
}
