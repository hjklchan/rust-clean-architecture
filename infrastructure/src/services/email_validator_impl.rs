use application::gateways::validations::email_validator::EmailValidator;
use emval::validate_email;

pub struct EmailValidatorImpl;

impl EmailValidator for EmailValidatorImpl {
    fn is_valid(&self, value: &str) -> bool {
        validate_email(value).is_ok()
    }
}
