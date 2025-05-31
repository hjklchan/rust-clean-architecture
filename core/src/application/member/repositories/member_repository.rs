use crate::domain::member::Member;

#[async_trait::async_trait]
pub trait MemberRepository: Send + Sync {
    async fn create_member(&self, member: Member) -> u64;
    async fn soft_delete_member(&self, id: u64);
    async fn forced_delete_member(&self, id: u64);
    async fn update_member(&self, member: Member);
    async fn get_member_by_id(&self, id: u64) -> Option<Member>;
    async fn get_member_by_name(&self, first_name: String, last_name: String) -> Option<Member>;
    async fn check_member_if_exists(&self, id: u64) -> bool;
}
