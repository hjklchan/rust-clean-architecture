
pub trait EmailValidator {
    fn is_valid_email(&self, email: &str) -> bool {
        true
    }
}