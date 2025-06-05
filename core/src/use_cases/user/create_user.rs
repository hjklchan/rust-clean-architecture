use chrono::Utc;
use thiserror::Error;

use crate::{
    entities::user::{Status, User},
    use_cases::interfaces::repositories::{UserRepository, error::UserRepositoryError},
};

pub struct CreateUserUseCase<'r, R1>
where
    R1: UserRepository,
{
    user_repo: &'r R1,
}

impl<'r, R1> CreateUserUseCase<'r, R1>
where
    R1: UserRepository,
{
    pub fn new(user_repo: &'r R1) -> Self {
        Self { user_repo }
    }
}

impl<'r, R1> CreateUserUseCase<'r, R1>
where
    R1: UserRepository,
{
    pub async fn execute(
        &self,
        input: inout_data::CreateUserInputData,
    ) -> Result<u64, CreateUserError> {
        if self
            .user_repo
            .check_if_email_is_in_use(&input.email)
            .await
            .map_err(|err| CreateUserError::from(err))?
        {
            return Err(CreateUserError::EmailAlreadyInUse);
        }

        let now_utc = Utc::now();

        let id = self
            .user_repo
            .save(User {
                id: None,
                username: input.username,
                email: input.email,
                hash_password: input.hash_password,
                status: Status::Normal,
                gender: input.gender.into(),
                created_at: now_utc,
                updated_at: now_utc,
                deleted_at: None,
            })
            .await
            .map_err(|err| CreateUserError::from(err))?;

        Ok(id)
    }
}

/// The error is present the business errors.
#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("{}", UserRepositoryError::DatasourceAccessError)]
    RepositoryError,
    #[error("The e-mail address has already been used")]
    EmailAlreadyInUse,
    // #[error("{}", UserRepositoryError::UserAlreadyExists)]
    // UserAlreadyExists,
    #[error("The user doesn't exist")]
    UserNotFound,
}

impl From<UserRepositoryError> for CreateUserError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::DatasourceAccessError => CreateUserError::RepositoryError,
        }
    }
}

pub mod inout_data {
    use crate::entities::user::{Gender, Status};

    #[derive(Debug, Default)]
    pub enum UserGender {
        #[default]
        Unknown,
        Private,
        Male,   // 🚹
        Female, // 🚺
    }

    impl From<UserGender> for Gender {
        fn from(value: UserGender) -> Self {
            match value {
                UserGender::Unknown => Self::Unknown,
                UserGender::Male => Self::Male,
                UserGender::Female => Self::Female,
                UserGender::Private => Self::Private,
            }
        }
    }

    #[derive(Debug, Default)]
    pub enum UserStatus {
        #[default]
        Normal,
        Frozen,
        Blocked,
    }

    impl From<UserStatus> for Status {
        fn from(value: UserStatus) -> Self {
            match value {
                UserStatus::Normal => Self::Normal,
                UserStatus::Blocked => Self::Blocked,
                UserStatus::Frozen => Self::Frozen,
            }
        }
    }

    pub struct CreateUserInputData {
        pub username: String,
        pub email: String,
        pub hash_password: String,
        pub gender: UserGender,
    }

    pub struct CreateUserOutputData {
        pub new_id: u64,
    }
}
