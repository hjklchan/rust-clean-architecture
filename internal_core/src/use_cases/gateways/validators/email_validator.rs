pub trait EmailValidator {
    fn is_valid_email(&self, #[allow(unused)] email: &str) -> bool {
        true
    }
}
