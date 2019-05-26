use super::super::lexer::Position;
use super::*;
use std::{iter::Peekable, slice::Iter};

#[cfg(test)]
mod tests;

pub struct AST {
    root: Box<Compound>,
}

#[derive(Debug, PartialEq)]
enum Compound {
    Some(Expression, Box<Compound>),
    None,
}

#[derive(Debug, PartialEq)]
enum Expression {
    QuoteExpression(Box<Expression>),
    List(List),
    Name(Position, String),
    Primitive(Position, Literal),
}

#[derive(Debug, PartialEq)]
struct List {
    left: Position,
    content: Box<Compound>,
    right: Position,
}

type Queue<'a> = Peekable<Iter<'a, Symbol>>;

trait Parsable {
    type Item;

    fn parse(input: &mut Queue) -> Result<Self::Item, error::ParserError>;
}

impl Parsable for Compound {
    type Item = Compound;

    fn parse(input: &mut Queue) -> Result<Self::Item, error::ParserError> {
        match input.peek() {
            None => Ok(Compound::None),
            Some(Symbol::RParen(_)) => Ok(Compound::None),
            Some(_) => Ok(Compound::Some(
                Expression::parse(input)?,
                Box::new(Compound::parse(input)?),
            )),
        }
    }
}

impl Parsable for Expression {
    type Item = Expression;

    fn parse(input: &mut Queue) -> Result<Self::Item, error::ParserError> {
        match input.peek() {
            Some(Symbol::Quote(_)) => {
                input.next();
                Ok(Expression::QuoteExpression(Box::new(Expression::parse(
                    input,
                )?)))
            }
            Some(Symbol::LParen(_)) => Ok(Expression::List(List::parse(input)?)),
            Some(Symbol::Name(p, n)) => {
                input.next();
                Ok(Expression::Name(*p, (*n).clone()))
            }
            Some(Symbol::Primitive(pos, prim)) => {
                input.next();
                Ok(Expression::Primitive(*pos, (*prim).clone()))
            }
            Some(s) => Err(error::ParserError::new(
                s.position(),
                format!("Expected: ', (, name, or primitive, found: {:?}", s),
            )),
            None => Err(error::ParserError::new(
                Position::at(0, 0), // TODO: Read previous Symbol
                String::from("Expected: ', (, name, or primitive, found: EOF"),
            )),
        }
    }
}

impl Parsable for List {
    type Item = List;

    fn parse(input: &mut Queue) -> Result<Self::Item, error::ParserError> {
        let left = match input.next() {
            Some(Symbol::LParen(p)) => *p,
            Some(s) => {
                return Err(error::ParserError::new(
                    s.position(),
                    format!("Expected: (, found: {:?}", input.peek().unwrap()),
                ))
            }
            None => {
                return Err(error::ParserError::new(
                    Position::at(0, 0), // TODO: Read previous Symbol
                    String::from("Expected: (, found: EOF"),
                ));
            }
        };
        let content = Box::new(match input.peek() {
            Some(_) => Compound::parse(input)?,
            None => {
                return Err(error::ParserError::new(
                    Position::at(0, 0), // TODO: Read previous Symbol
                    String::from("Expected: expression, or ), found: EOF"),
                ));
            }
        });
        let right = match input.next() {
            Some(Symbol::RParen(p)) => *p,
            Some(s) => {
                return Err(error::ParserError::new(
                    s.position(), // TODO: Read previous Symbol
                    format!("Expected: ), found: {:?}", s),
                ));
            }
            None => {
                return Err(error::ParserError::new(
                    Position::at(0, 0), // TODO: Read previous Symbol
                    String::from("Expected: ), found: EOF"),
                ));
            }
        };

        Ok(List {
            left,
            content,
            right,
        })
    }
}
