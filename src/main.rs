mod ir;
mod lexer;
mod parser;
mod query;

use crate::{
    lexer::Token,
    parser::parser_pool::{Lang, ParserPool},
    query::{query_engine::QueryEngine, walking_engine::WalkingEngine},
};
use anyhow::{Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use logos::Logos;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    query: String,
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn lang_for_path(path: &std::path::Path) -> Option<Lang> {
    match path.extension()?.to_str()? {
        "rs" => Some(Lang::Rust),
        "ts" => Some(Lang::TypeScript),
        "tsx" => Some(Lang::Tsx),
        "js" => Some(Lang::JavaScript),
        _ => None,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lexer = Token::lexer(&args.query);
    let mut query_parser = crate::parser::Parser::new(lexer);
    let query = query_parser.parse_query()?;

    let engine = WalkingEngine {};

    let mut parsers = ParserPool::new();

    for entry in WalkBuilder::new(&args.path).build() {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let lang = lang_for_path(path);
        if lang.is_none() {
            continue;
        }
        let lang = lang.unwrap();

        let source = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let ts_parser = parsers.parser_for(lang)?;

        let tree = ts_parser
            .parse(&source, None)
            .context("tree-sitter failed to parse source")?;

        let matches = engine.run(&query, &tree, &source);

        for m in matches {
            let span = &source[m.start_byte..m.end_byte];
            println!("{:?}", span);
        }
    }

    Ok(())
}
