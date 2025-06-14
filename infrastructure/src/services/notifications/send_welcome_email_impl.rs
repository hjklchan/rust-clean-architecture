use application::gateways::notifications::send_welcome_email::SendWelcomeEmail;

pub struct SendWelcomeEmailImpl {}

#[async_trait::async_trait]
impl SendWelcomeEmail for SendWelcomeEmailImpl {
    async fn send(
        &self,
        email_address: &str,
        username: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
