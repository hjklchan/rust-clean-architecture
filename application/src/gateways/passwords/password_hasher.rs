use std::error::Error;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, Box<dyn Error>>;
    fn verify(&self, request_password: &str, password_hash: &str) -> Result<bool, Box<dyn Error>>;
}
