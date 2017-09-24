use std::str;
use mison::parser;
use mison::query::QueryTree;
use mison::index_builder::backend::Sse2Backend;
use mison::errors::Result;
use super::Parser;


pub struct MisonParser<'a> {
    pikkr: parser::Parser<'a, Sse2Backend>,
}

impl<'a> MisonParser<'a> {
    pub fn new(queries: &[&'a str]) -> Result<Self> {
        let mut tree = QueryTree::default();
        for q in queries {
            tree.add_path(q)?;
        }
        let index_builder = Default::default();
        Ok(MisonParser { pikkr: parser::Parser::new(tree, index_builder) })
    }
}

impl<'a> Parser for MisonParser<'a> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = self.pikkr.parse(rec).unwrap();

        let mut r = 0;
        for x in v {
            let x = x.unwrap();
            r += x.len();
            if print {
                println!("{}", x);
            }
        }
        r
    }
}
