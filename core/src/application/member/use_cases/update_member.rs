use crate::{
    application::member::repositories::member_repository::MemberRepository, domain::member::Member,
};

pub struct UpdateMemberUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> UpdateMemberUseCase<R> {
    pub async fn execute(&self, member: Member) {
        self.member_repo.update_member(member).await;
    }
}
