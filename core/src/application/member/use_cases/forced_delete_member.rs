use crate::application::member::repositories::member_repository::MemberRepository;

pub struct ForcedDeleteMemberUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> ForcedDeleteMemberUseCase<R> {
    pub async fn execute(&self, id: u64) {
        let member = self.member_repo.get_member_by_id(id).await;

        // Ensure the user was exited.
        if let Some(member) = member {
            // Check whether the user can be deleted forcibly.
            if member.can_forced_delete() {
                self.member_repo.forced_delete_member(id).await;
            }
        }
    }
}
