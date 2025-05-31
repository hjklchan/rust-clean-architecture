use crate::{
    application::member::repositories::member_repository::MemberRepository, domain::member::Member,
};

pub struct CreateMemberUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> CreateMemberUseCase<R> {
    pub async fn execute(&self, member: Member) -> u64 {
        let new_id = self.member_repo.create_member(member).await;
        new_id
    }
}
