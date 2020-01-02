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

use super::lexer::Symbol;

#[cfg(test)]
mod tests;

pub mod ast;
pub mod error;

/// Parses a `Vec<lexer::Symbol>` into an `parser::ast::AST`
///
/// TODO: Add an example, and explanation of the AST
pub fn parse(symbols: &mut Vec<Symbol>) -> Result<ast::AST, error::ParserError> {
    let ast = ast::AST::parse(&mut symbols.iter().peekable())?;
    if let ast::Compound::None = *ast.root {
        if let Some(s) = symbols.first() {
            return Err(error::ParserError::new(
                s.position(),
                format!(
                    "Expected (, name, string, number, or boolean, found: {:?}",
                    s
                ),
            ));
        }
    }
    Ok(ast)
}
