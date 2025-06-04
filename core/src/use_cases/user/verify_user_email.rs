use crate::use_cases::interfaces::validators::email_validator::EmailValidator;


pub struct VerifyUserEmail<V: EmailValidator> {
    email_validator: V,
}

impl<V: EmailValidator> VerifyUserEmail<V> {
    fn execute(&self, email: String) -> bool {
        self.email_validator.is_valid_email(&email)
    }
}