#![allow(unused)]

use regex::Regex;

/**
* Behold, the Luy Intermediate Representation (LIR).
* Luy takes a user's query and converts it into LIR
* which is then piped to the query engine.
*/

#[derive(Clone, Debug)]
pub enum Kind {
    Function,
    Var,
    Comment,

    Call,
    Param,
    Arg,
    String,
    Import,
    Type,
}

#[derive(Clone, Debug)]
pub enum Field {
    Self_,
    Name,
    Body,
    Params,
    Args,
    Type,
    ReturnType,

    Value,
    Receiver,

    Path,

    Condition,
}

#[derive(Clone, Debug)]
pub enum Query {
    Pattern(Pattern),
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub kind: Kind,
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub field: Field,
    pub predicate: Predicate,
}

#[derive(Clone, Debug)]
pub struct ContainsText {
    pub case_sensitive: bool,
    pub text: String,
}

#[derive(Clone, Debug)]
pub enum Predicate {
    Eq(String),
    Matches(Regex),
    ContainsText(ContainsText),
    ContainsPattern(Box<Pattern>),
}
