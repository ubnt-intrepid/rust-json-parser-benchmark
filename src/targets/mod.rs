pub mod serde_json;
pub mod json;
pub mod pikkr;
pub mod mison;

pub trait Parser {
    fn parse(&mut self, rec: &str, print: bool) -> usize;
}
