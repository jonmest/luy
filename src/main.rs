mod core;
mod lexer;
mod parser;
use logos::Logos;
use tree_sitter::{Language, Node, Parser, Query};

use crate::lexer::Token;

fn main() {
    // "fn[name like cool]"
    let lexer = Token::lexer("fn[name like 'cool']");
    let mut p = parser::Parser::new(lexer);
    let r = p.parse_query().unwrap();
    println!("{:?}", r);
}
