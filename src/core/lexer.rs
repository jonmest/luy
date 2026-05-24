use logos::Logos;

fn unquote(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    // kinds
    #[token("fn")]
    Fn,
    #[token("var")]
    Var,
    #[token("comment")]
    Comment,
    #[token("call")]
    Call,
    #[token("param")]
    Param,
    #[token("arg")]
    Arg,
    #[token("string")]
    String,
    #[token("import")]
    Import,
    #[token("type")]
    Type,

    // symbols
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[regex(r"\r?\n")]
    Newline,

    // fields
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
    #[token("returns")]
    ReturnType,

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
    #[token("contains")]
    Contains,

    #[token("all")]
    All,
    #[token("count")]
    Count,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| unquote(lex.slice()))]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| unquote(lex.slice()))]
    Text(String),
}
