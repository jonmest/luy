use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
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
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("\"")]
    #[token("'")]
    Quote,
    #[token(",")]
    Comma,

    #[token("name")]
    Name,

    #[token("=")]
    Equals,

    #[token("matches")]
    Matches,
    #[token("like")]
    Like,

    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())]
    String(String),
}
