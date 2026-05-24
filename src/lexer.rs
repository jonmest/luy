use logos::Logos;

fn unquote(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    // entities
    #[token("fn")]
    Fn,
    #[token("var")]
    Var,
    #[token("comment")]
    Comment,

    // relations
    #[token("in")]
    In,
    #[token("to")]
    To,
    #[token("before")]
    Before,

    // symbols
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[regex(r"\r?\n")]
    Newline,

    // properties
    #[token("params")]
    Params,

    #[token("args")]
    Args,

    #[token("value")]
    Value,

    #[token("name")]
    Name,

    #[token("body")]
    Body,

    #[token("=")]
    Equals,

    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    #[token("matches")]
    Matches,

    #[token("any")]
    Any,
    #[token("all")]
    All,
    #[token("count")]
    Count,
    #[token("contains")]
    Contains,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| unquote(lex.slice()))]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| unquote(lex.slice()))]
    String(String),
}
