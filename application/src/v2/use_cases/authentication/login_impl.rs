use domain::repositories::user_repository::UserRepository;

use crate::{
    gateways::{
        authentication::jwt_token_generator::JwtTokenGenerator,
        passwords::password_hasher::PasswordHasher,
    },
    v2::use_cases::authentication::login::{
        LoginInputData, LoginOutputData, LoginUseCase, LoginUseCaseError,
    },
};

pub struct LoginUseCaseImpl<R, S1, S2>
where
    R: UserRepository,
    S1: JwtTokenGenerator,
    S2: PasswordHasher,
{
    user_repo: R,
    jwt_token_generator: S1,
    password_hasher: S2,
}

#[async_trait::async_trait]
impl<R, S1, S2> LoginUseCase for LoginUseCaseImpl<R, S1, S2>
where
    R: UserRepository,
    S1: JwtTokenGenerator,
    S2: PasswordHasher,
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
            if !self
                .password_hasher
                .verify(&input.password, &user.password)
                .map_err(|err| LoginUseCaseError::ParsePasswordError(err))?
            {
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
