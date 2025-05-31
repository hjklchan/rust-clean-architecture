use crate::application::member::repositories::member_repository::MemberRepository;

pub struct GetMemberByIdUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> GetMemberByIdUseCase<R> {
    pub async fn execute(&self, id: u64) {
        let _member = self.member_repo.get_member_by_id(id).await;
    }
}
