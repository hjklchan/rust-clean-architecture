use std::error::Error;

pub trait JwtTokenParser: Send + Sync {
    fn parse<T>(&self, token: &str) -> Result<(T, &str), Box<dyn Error>>
    where
        T: Copy + Clone + Eq + Ord + PartialEq + PartialOrd;
}
