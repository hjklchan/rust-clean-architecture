use crate::use_cases::gateways::validators::email_validator::EmailValidator;


#[allow(unused)]
pub struct VerifyUserEmail<V: EmailValidator> {
    email_validator: V,
}

#[allow(unused)]
impl<V: EmailValidator> VerifyUserEmail<V> {
    fn execute(&self, email: String) -> bool {
        self.email_validator.is_valid_email(&email)
    }
}

pub enum VerifyUserEmailError {
    DomainDoesNotExist,
    AddressDoesNotExist,
    // more validation errors...
}