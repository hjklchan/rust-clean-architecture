use crate::application::member::repositories::member_repository::MemberRepository;

pub struct SoftDeleteMemberUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> SoftDeleteMemberUseCase<R> {
    pub async fn execute(&self, id: u64) {
        let old = self.member_repo.get_member_by_id(id).await;

        if let Some(mut member) = old {
            if !member.is_soft_deleted() {
                member.fill_delete_at();
            }
        }
    }
}
