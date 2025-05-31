use crate::application::member::repositories::member_repository::MemberRepository;

pub struct GetMemberByNameUseCase<R: MemberRepository> {
    member_repo: R,
}

impl<R: MemberRepository> GetMemberByNameUseCase<R> {
    pub async fn execute(&self, first_name: String, last_name: String) {
        let _member = self
            .member_repo
            .get_member_by_name(first_name, last_name)
            .await;

        todo!("who will adapter and implement this output port data?");
    }
}
