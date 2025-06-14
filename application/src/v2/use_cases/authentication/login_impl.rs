use domain::repositories::user_repository::UserRepository;
use uuid::Uuid;

use crate::{
    gateways::authentication::jwt_token_generator::JwtTokenGenerator,
    v2::use_cases::authentication::login::{
        LoginInputData, LoginOutputData, LoginUseCase, LoginUseCaseError,
    },
};

pub struct LoginUseCaseImpl<R, S1>
where
    R: UserRepository,
    S1: JwtTokenGenerator,
{
    user_repo: R,
    jwt_token_generator: S1,
}

#[async_trait::async_trait]
impl<R, S1> LoginUseCase for LoginUseCaseImpl<R, S1>
where
    R: UserRepository,
    S1: JwtTokenGenerator,
{
    async fn execute(&self, input: LoginInputData) -> Result<LoginOutputData, LoginUseCaseError> {
        // get user by email and password
        let user = self
            .user_repo
            .get_by_email(&input.email)
            .await
            .map_err(|err| LoginUseCaseError::Repository(err))?;

        if let Some(user) = user {
            // verify the password
            if user.password != input.password {
                return Err(LoginUseCaseError::IncorrectPassword);
            }

            // generate jwt token
            let token = self
                .jwt_token_generator
                .generate(user.id.as_ref(), &user.username)
                .map_err(|err| LoginUseCaseError::JwtGenerateError(err))?;

            Ok(LoginOutputData {
                id: user.id.to_string(),
                token,
            })
        } else {
            Err(LoginUseCaseError::UserNotFound)
        }
    }
}
