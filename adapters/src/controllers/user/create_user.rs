use core::use_cases::{
    interfaces::repositories::UserRepository,
    user::create_user::{
        CreateUserUseCase,
        inout_data::{CreateUserInputData, CreateUserOutputData},
    },
};

use crate::interfaces::presenter::Presenter;

pub struct CreateUserController<R1, P>
where
    R1: UserRepository,
    P: Presenter<Result<CreateUserOutputData, ()>>,
{
    user_repo: R1,
    presenter: P,
}

impl<R1, P> CreateUserController<R1, P>
where
    R1: UserRepository,
    P: Presenter<Result<CreateUserOutputData, ()>>,
{
    pub fn new(user_repo: R1, presenter: P) -> Self {
        Self {
            user_repo,
            presenter,
        }
    }
}

impl<R1, P> CreateUserController<R1, P>
where
    R1: UserRepository,
    P: Presenter<Result<CreateUserOutputData, ()>>,
{
    pub async fn create_user(&self, request: impl Into<CreateUserInputData>) -> P::PresentModel {
        let d = CreateUserUseCase::new(&self.user_repo)
            .execute(request.into())
            .await;

        self.presenter
            .present_value(Ok(CreateUserOutputData { new_id: 0 }))
    }
}
