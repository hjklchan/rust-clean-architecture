use uuid::Uuid;

use crate::{
    gateways::{
        authentication::jwt_token_generator::JwtTokenGenerator,
        notifications::send_welcome_email::SendWelcomeEmail,
    },
    v2::use_cases::authentication::register::{
        RegisterInputData, RegisterOutputData, RegisterUseCase, RegisterUseCaseError,
    },
};
use domain::{
    repositories::user_repository::UserRepository,
    v2::user::{User, value_objects::user_id::UserId},
};

pub struct RegisterUseCaseImpl<R, S1, S2>
where
    R: UserRepository,
    S1: SendWelcomeEmail,
    S2: JwtTokenGenerator,
{
    user_repo: R,
    send_welcome_email: S1,
    jwt_token_generator: S2,
}

#[async_trait::async_trait]
impl<R, S1, S2> RegisterUseCase for RegisterUseCaseImpl<R, S1, S2>
where
    R: UserRepository,
    S1: SendWelcomeEmail,
    S2: JwtTokenGenerator,
{
    async fn execute(
        &self,
        input: RegisterInputData,
    ) -> Result<RegisterOutputData, RegisterUseCaseError> {
        let RegisterInputData {
            username,
            email,
            password,
        } = input;

        // TODO need to check whether the email is already in use.
        // TODO need to check whether the username is already in use.

        let id = self
            .user_repo
            .save(User::new(
                UserId::generate(),
                username.clone(),
                email.clone(),
                password,
            ))
            .await
            .map_err(|err| RegisterUseCaseError::Repository(err.into()))?;

        // send welcome email after the user registration is completed.
        self.send_welcome_email
            .send(&email, Some(&username))
            .await
            .map_err(|err| RegisterUseCaseError::SendWelcomeEmailError(err))?;

        // generate the jwt token
        let token = self.jwt_token_generator
            .generate(&id, &username)
            .map_err(|err| RegisterUseCaseError::JwtGenerateError(err))?;

        Ok(RegisterOutputData {
            id: id.to_string(),
            token,
        })
    }
}
