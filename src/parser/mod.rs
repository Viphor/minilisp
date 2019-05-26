//! This module exposes the `parse` function.
//!
//! # Syntax
//! The parser will parse the following EBNF:
//!
//! ```EBNF
//! START ::= COMPOUND $
//! COMPOUND ::= EXPRESSION COMPOUND
//!            | λ
//! EXPRESSION ::= Quote EXPRESSION
//!              | LIST
//!              | PRIMITIVE
//!              | Name
//! LIST ::= LParen COMPOUND RParen
//! PRIMITIVE ::= String
//!             | Number
//!             | Boolean
//! ```

use super::lexer::{Literal, Symbol};

#[cfg(test)]
mod tests;

mod ast;
mod error;

//pub fn parse(symbols: Vec<Symbol>) -> ast::AST {
//    ast::AST {
//        root: Box::new(ast::Compound::None)
//    }
//}
