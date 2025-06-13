use uuid::Uuid;

use crate::v2::use_cases::authentication::register::{
    RegisterInputData, RegisterOutputData, RegisterUseCase, RegisterUseCaseError,
};

pub struct RegisterUseCaseImpl {}

#[async_trait::async_trait]
impl RegisterUseCase for RegisterUseCaseImpl {
    async fn execute(
        &self,
        input: RegisterInputData,
    ) -> Result<RegisterOutputData, RegisterUseCaseError> {
        Ok(RegisterOutputData {
            id: Uuid::new_v4().to_string(),
            token: "xxxTOKENxxx".to_string(),
        })
    }
}
