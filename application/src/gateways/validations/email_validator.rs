pub trait EmailValidator {
    fn is_valid(&self, value: &str) -> bool;
}

#[async_trait::async_trait]
pub trait EmailValidatorAsync {
    async fn is_valid(&self, value: &str) -> bool;
}
