use std::error::Error;

#[async_trait::async_trait]
pub trait SendWelcomeEmail: Send + Sync {
    async fn send(&self, email_address: &str, username: Option<&str>)
    -> Result<(), Box<dyn Error>>;
}
