fn main() {
    println!("Hello, world!");
}

trait CreateUserRepository {}

trait UpdateUserRepository {}

struct MysqlCreateUserRepository;

impl CreateUserRepository for MysqlCreateUserRepository {}

struct MysqlUpdateUserRepository;

impl UpdateUserRepository for MysqlUpdateUserRepository {}

trait UserRepository {
    fn create_user(&self) -> impl CreateUserRepository;
    fn update_user(&self) -> impl UpdateUserRepository;
}

pub struct MysqlUserRepository;

impl UserRepository for MysqlUserRepository {
    fn create_user(&self) -> impl CreateUserRepository {
        MysqlCreateUserRepository
    }

    fn update_user(&self) -> impl UpdateUserRepository {
        MysqlUpdateUserRepository
    }
}