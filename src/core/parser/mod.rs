#![allow(unused)]
pub mod parser_pool;

use crate::core::{
    ir::{ContainsText, Field, Filter, Kind, Pattern, Predicate, Query},
    lexer::Token,
};
use anyhow::{Error, Result, anyhow};
use logos::Lexer;

enum ParsedValue {
    String(String),
    Pattern(Pattern),
}

enum FieldParsingError {
    NotField,
    InvalidToken,
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

        let filters = if matches!(self.current, Some(Ok(Token::LeftBrace))) {
            self.advance();
            let filters = self.parse_filters()?;
            self.expect(Token::RightBrace)?;
            filters
        } else if self.current.is_some() {
            vec![self.parse_filter()?]
        } else {
            vec![]
        };

        Ok(Pattern { kind, filters })
    }

    fn parse_kind(&mut self) -> Result<Kind> {
        let current = &self.current;
        match current {
            Some(Ok(Token::Fn)) => {
                self.advance();
                Ok(Kind::Function)
            }
            Some(Ok(Token::Comment)) => {
                self.advance();
                Ok(Kind::Comment)
            }
            _ => Err(anyhow!("invalid kind")),
        }
    }

    fn consume_separators(&mut self) {
        while matches!(
            self.current,
            Some(Ok(Token::Comma)) | Some(Ok(Token::Newline))
        ) {
            self.advance();
        }
    }

    fn parse_filters(&mut self) -> Result<Vec<Filter>> {
        let mut filters = Vec::new();

        self.consume_separators();

        while !matches!(self.current, Some(Ok(Token::RightBrace))) {
            filters.push(self.parse_filter()?);
            self.consume_separators();
        }

        Ok(filters)
    }

    fn parse_filter(&mut self) -> Result<Filter> {
        let field = match self.parse_field() {
            Ok(field) => Ok(field),
            Err(FieldParsingError::NotField) => Ok(Field::Self_),
            Err(_) => Err(anyhow!("Failed to parse field")),
        }?;
        let predicate = self.parse_predicate()?;

        Ok(Filter { field, predicate })
    }

    fn parse_field(&mut self) -> Result<Field, FieldParsingError> {
        match &self.current {
            Some(Ok(Token::Name)) => {
                self.advance();
                Ok(Field::Name)
            }
            Some(Ok(Token::Params)) => {
                self.advance();
                Ok(Field::Params)
            }
            Some(Ok(Token::Body)) => {
                self.advance();
                Ok(Field::Body)
            }
            Some(Ok(token)) => Err(FieldParsingError::NotField),
            _ => Err(FieldParsingError::InvalidToken),
        }
    }

    fn parse_predicate(&mut self) -> Result<Predicate> {
        let current = &self.current;
        match current {
            Some(Ok(Token::Equals)) => {
                self.advance();
                match self.parse_value()? {
                    ParsedValue::String(value) => Ok(Predicate::Eq(value)),
                    _ => Err(anyhow!("equals operator expects string value")),
                }
            }
            Some(Ok(Token::Matches)) => {
                self.advance();
                match self.parse_value()? {
                    ParsedValue::String(value) => Ok(Predicate::Matches(value)),
                    _ => Err(anyhow!("equals operator expects string value")),
                }
            }
            Some(Ok(Token::All)) => {
                self.advance();
                let predicate = self.parse_predicate()?;
                Ok(Predicate::All(Box::new(predicate)))
            }
            Some(Ok(Token::Contains)) => {
                self.advance();
                match self.parse_value()? {
                    ParsedValue::String(val) => Ok(Predicate::ContainsText(ContainsText {
                        text: val,
                        case_sensitive: true,
                    })),
                    ParsedValue::Pattern(pattern) => {
                        Ok(Predicate::ContainsPattern(Box::new(pattern)))
                    }
                }
            }
            Some(t) => Err(anyhow!("expected predicate got {:?}", t)),
            _ => Err(anyhow!("expected predicate")),
        }
    }

    fn parse_value(&mut self) -> Result<ParsedValue> {
        match &self.current {
            Some(Ok(Token::String(value))) => {
                let value = value.clone();
                self.advance();
                Ok(ParsedValue::String(value))
            }
            _ => {
                let pattern = self.parse_pattern()?;
                Ok(ParsedValue::Pattern(pattern))
            }
        }
    }
}
