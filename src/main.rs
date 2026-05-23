mod ir;
mod lexer;
mod parser;
mod query;
use logos::Logos;
use tree_sitter::{Language, Node, Parser, Query};

use crate::lexer::Token;

fn main() {
    // "fn[name like cool, ]"
    let lexer = Token::lexer("fn[name = 'cool', params grep 'user_']");
    let mut p = parser::Parser::new(lexer);
    let r = p.parse_query().unwrap();
    println!("{:?}", r);
}
