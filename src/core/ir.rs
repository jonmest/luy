#![allow(unused)]

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
}

#[derive(Clone, Debug)]
pub enum Field {
    Self_,
    Name,
    Body,
    Params,
    Arg,
}

#[derive(Clone, Debug)]
pub enum Query {
    Pattern(Pattern),
    Not(Box<Query>),
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    In(Box<Query>, Box<Query>),
    To(Box<Query>, Box<Query>),
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
    Matches(String),
    ContainsText(ContainsText),
    ContainsPattern(Box<Pattern>),
    All(Box<Predicate>),
}
