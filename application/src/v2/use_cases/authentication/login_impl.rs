use uuid::Uuid;

use crate::v2::use_cases::authentication::login::{
    LoginInputData, LoginOutputData, LoginUseCase, LoginUseCaseError,
};

pub struct LoginUseCaseImpl {}

#[async_trait::async_trait]
impl LoginUseCase for LoginUseCaseImpl {
    async fn execute(&self, input: LoginInputData) -> Result<LoginOutputData, LoginUseCaseError> {
        Ok(LoginOutputData {
            id: Uuid::new_v4().to_string(),
            token: "xxxTOKENxxx".to_string(),
        })
    }
}
