use std::error::Error;

pub trait JwtTokenGenerator: Send + Sync {
    fn generate<T>(&self, id: &T, username: &str) -> Result<String, Box<dyn Error>>
    where
        T: Copy + Clone + Eq + Ord + PartialEq + PartialOrd;
}
