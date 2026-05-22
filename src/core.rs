pub enum Language {
    Typescript,
}

pub enum Kind {
    Function,

    Variable,
    Call,

    Import,
    Export,

    Literal,
}

pub struct Entity {
    kind: Kind,
    name: Option<String>,
    language: Option<Language>,
    value: Option<String>,
}

// Entity Properties Relationship
// PATTERN
// entities - relations
