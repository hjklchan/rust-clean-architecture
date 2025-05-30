use crate::domain::entities::article::value_objects::Status;

pub struct ChangeArticleStatusUseCase {}

impl ChangeArticleStatusUseCase {
    pub fn execute(&self, status: Status) {}
}
