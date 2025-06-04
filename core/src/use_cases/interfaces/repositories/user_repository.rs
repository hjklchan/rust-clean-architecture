use crate::entities::user::User;


pub trait UserRepository {
    fn save(&self, user: User) -> u64;
    fn soft_delete(&self, id: u64);
    fn forced_delete(&self, id: u64);
    fn get_by_id(&self, id: u64) -> User;
}