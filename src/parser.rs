use crate::lexer::Token;
use anyhow::{Error, Result, anyhow};
use logos::Lexer;

#[derive(Clone, Debug)]
pub enum Kind {
    Function,
    Var,
    Comment,
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
    kind: Kind,
    filters: Vec<Filter>,
}

#[derive(Clone, Debug)]
pub struct Filter {
    field: String,
    predicate: Predicate,
}

#[derive(Clone, Debug)]
pub enum Predicate {
    Eq(String),
    Like(String),
    Matches(String),
    Contains(Box<Pattern>),
}

pub struct Parser<'source> {
    lexer: logos::Lexer<'source, Token>,
    current: Option<Result<Token, ()>>,
}

impl<'source> Parser<'source> {
    pub fn new(mut lexer: logos::Lexer<'source, Token>) -> Self {
        let current = lexer.next();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        let current = &self.current;
        println!("{:?}", current);
        match current {
            Some(Ok(token)) if *token == expected => {
                self.advance();
                Ok(())
            }
            Some(Err(token)) => Err(anyhow!("expected {:?}, got {:?}", expected, token)),
            _ => Err(anyhow!("expected {:?}", expected)),
        }
    }

    pub fn parse_query(&mut self) -> Result<Query> {
        let pattern = self.parse_pattern()?;
        Ok(Query::Pattern(pattern))
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        let kind = self.parse_kind()?;
        self.expect(Token::LeftBracket)?;
        let filters = self.parse_filters()?;
        self.expect(Token::RightBracket)?;
        Ok(Pattern { kind, filters })
    }

    fn parse_kind(&mut self) -> Result<Kind> {
        let current = &self.current;
        match current {
            Some(Ok(Token::Fn)) => {
                self.advance();
                Ok(Kind::Function)
            }
            _ => Err(anyhow!("invalid kind")),
        }
    }

    fn parse_filters(&mut self) -> Result<Vec<Filter>> {
        let mut filters = Vec::new();
        while !matches!(self.current, Some(Ok(Token::RightBracket))) {
            filters.push(self.parse_filter()?);
            if matches!(self.current, Some(Ok(Token::Comma))) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(filters)
    }

    fn parse_filter(&mut self) -> Result<Filter> {
        let field = self.parse_field()?;
        let predicate = self.parse_predicate()?;

        Ok(Filter { field, predicate })
    }

    fn parse_field(&mut self) -> Result<String> {
        match &self.current {
            Some(Ok(Token::Name)) => {
                self.advance();
                Ok("name".to_string())
            }
            _ => Err(anyhow!("invalid field")),
        }
    }

    fn parse_predicate(&mut self) -> Result<Predicate> {
        let current = &self.current;
        println!("{:?}", current);
        match current {
            Some(Ok(Token::Like)) => {
                self.advance();
                let value = self.parse_value()?;
                Ok(Predicate::Like(value))
            }
            Some(Ok(Token::Equals)) => {
                self.advance();
                let value = self.parse_value()?;
                Ok(Predicate::Eq(value))
            }
            Some(Ok(Token::Matches)) => {
                self.advance();
                let value = self.parse_value()?;
                Ok(Predicate::Matches(value))
            }
            _ => Err(anyhow!("expected predicate")),
        }
    }

    fn parse_quote(&mut self) -> Result<()> {
        match &self.current {
            Some(Ok(Token::Quote)) => {
                self.advance();
                Ok(())
            }
            _ => Err(anyhow!("expected a quote")),
        }
    }

    fn parse_value(&mut self) -> Result<String> {
        self.parse_quote()?;
        let result = self.parse_string();
        self.parse_quote()?;
        result
    }

    fn parse_string(&mut self) -> Result<String> {
        match &self.current {
            Some(Ok(Token::String(value))) => {
                let value = value.clone();
                self.advance();
                Ok(value)
            }
            _ => Err(anyhow!("expected string")),
        }
    }
}
