use std::collections::HashMap;
use tree_sitter::Parser;

use crate::core::language::lang::Lang;

/*
*
* Store and reuse language-speicific parsers.
*
*/

pub struct ParserPool {
    parsers: HashMap<Lang, Parser>,
}

impl ParserPool {
    pub fn new() -> Self {
        ParserPool {
            parsers: HashMap::new(),
        }
    }
    pub fn parser_for(&mut self, lang: Lang) -> anyhow::Result<&mut Parser> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.parsers.entry(lang) {
            let mut parser = Parser::new();
            match lang {
                Lang::Rust => {
                    parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;
                }
                Lang::JavaScript => {
                    parser.set_language(&tree_sitter_javascript::LANGUAGE.into())?;
                }
                Lang::TypeScript => {
                    parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())?;
                }
                Lang::Tsx => {
                    parser.set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())?;
                }
                _ => todo!(),
            }
            e.insert(parser);
        }
        Ok(self.parsers.get_mut(&lang).unwrap())
    }
}
