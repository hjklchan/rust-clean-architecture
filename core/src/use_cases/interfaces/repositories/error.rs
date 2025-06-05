use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    // #[error("The e-mail address has already been used")]
    // EmailAlreadyInUse,
    #[error("The user already exists")]
    UserAlreadyExists,
    // #[error("The user doesn't exist")]
    // UserNotFound,

    #[error("An error occurred while accessing the data")]
    DatasourceAccessError,
}
