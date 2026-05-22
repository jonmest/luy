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
    pub kind: Kind,
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub field: String,
    pub predicate: Predicate,
}

#[derive(Clone, Debug)]
pub enum Predicate {
    Eq(String),
    Like(String),
    Matches(String),

    Any(Box<Predicate>),
    All(Box<Predicate>),

    Grep(String),
    Regex(String),
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

        if self.current.is_some() {
            return Err(anyhow!("unexpected token after query: {:?}", self.current));
        }
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
            Some(Ok(Token::Params)) => {
                self.advance();
                Ok("params".to_string())
            }
            _ => Err(anyhow!("invalid field")),
        }
    }

    fn parse_predicate(&mut self) -> Result<Predicate> {
        let current = &self.current;
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
            Some(Ok(Token::Any)) => {
                self.advance();
                let predicate = self.parse_predicate()?;
                Ok(Predicate::Any(Box::new(predicate)))
            }
            Some(Ok(Token::All)) => {
                self.advance();
                let predicate = self.parse_predicate()?;
                Ok(Predicate::All(Box::new(predicate)))
            }
            Some(Ok(Token::Grep)) => {
                self.advance();
                let m = self.parse_value()?;
                Ok(Predicate::Grep(m))
            }
            Some(Ok(Token::Regex)) => {
                self.advance();
                let regex_string = self.parse_value()?;
                Ok(Predicate::Regex(regex_string))
            }
            Some(t) => Err(anyhow!("expected predicate got {:?}", t)),
            _ => Err(anyhow!("expected predicate")),
        }
    }

    fn parse_value(&mut self) -> Result<String> {
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
