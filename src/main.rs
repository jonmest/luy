mod core;

use crate::core::{
    language::{lang::Lang, spec_pool::LanguageSpecPool},
    lexer::Token,
    parser::{Parser as LuyParser, parser_pool::ParserPool},
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

    let mut spec_pool = LanguageSpecPool::new();

    let lexer = Token::lexer(&args.query);
    let mut query_parser = LuyParser::new(lexer);
    let query = query_parser.parse_query()?;

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

        let spec = spec_pool.spec_for(lang);
        if spec.is_err() {
            continue;
        }
        let spec = spec.unwrap();

        let engine = WalkingEngine::new(spec, &query);
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let ts_parser = parsers.parser_for(lang)?;

        let tree = ts_parser
            .parse(&source, None)
            .context("tree-sitter failed to parse source")?;

        let matches = engine.run(&tree, &source);

        for m in matches {
            println!(
                "{}:{}:{}\n{}",
                path.canonicalize()?.display(),
                m.start_row,
                m.start_col,
                m.text
            );
        }
    }

    Ok(())
}
