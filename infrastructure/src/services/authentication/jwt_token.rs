use application::gateways::authentication::{
    jwt_token_generator::JwtTokenGenerator, jwt_token_parser::JwtTokenParser,
};

pub struct JwtTokenGeneratorImpl {}

impl JwtTokenGenerator for JwtTokenGeneratorImpl {
    fn generate<T>(&self, id: &T, username: &str) -> Result<String, Box<dyn std::error::Error>>
    where
        T: Copy + Clone + Eq + Ord + PartialEq + PartialOrd,
    {
        todo!()
    }
}

pub struct JwtTokenParserImpl {}

impl JwtTokenParser for JwtTokenGeneratorImpl {
    fn parse<T>(&self, token: &str) -> Result<(T, &str), Box<dyn std::error::Error>>
    where
        T: Copy + Clone + Eq + Ord + PartialEq + PartialOrd,
    {
        todo!()
    }
}
