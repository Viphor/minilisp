//! This module mainly contains the function `lex`, which can take a string
//! reference and convert it into a vector of symbols.

#[cfg(test)]
mod tests;

pub mod error;
mod tracking;

pub use tracking::Position;

/// Enum over the different symbols we can lex.
///
/// Each symbol contains a `Position` struct, which denotes the
/// position of the first character of the symbol.
#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    /// Represents the quote `'`
    Quote(Position),
    /// Represents the left parenthesis `(`
    LParen(Position),
    /// Represents the right parenthesis `)`
    RParen(Position),
    /// Represents bound names
    /// ### Syntax
    /// Matched by the following regex: `[^"#0-9\s][^"\s]*`
    Name(Position, String),
    /// Represents any of the literals defined in `enum Literal`
    Primitive(Position, Literal),
}

impl Symbol {
    /// Extracts the position of a `Symbol`.
    pub fn position(&self) -> Position {
        *match self {
            Symbol::Quote(p) => p,
            Symbol::LParen(p) => p,
            Symbol::RParen(p) => p,
            Symbol::Name(p, _) => p,
            Symbol::Primitive(p, _) => p,
        }
    }
}

/// Enum over the literal types that we can lex.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// This literal encodes numbers. Currently it only supports integers.
    /// ### Syntax
    /// The symbol is matched by the following regex: `-?[0-9]+`
    Number(i64),
    /// This literal encodes boolean values.
    /// ### Syntax
    /// The symbol is matched by the following regex: `#[tf]`
    Boolean(bool),
    /// This literal encodes string values.
    /// ### Syntax
    /// The symbol is matched by the following regex: `"(.*(\\")?)*"`
    String(String),
    /// This literal is currently not supported yet.
    None,
}

struct Buffers {
    symbols: Vec<Symbol>,
    buffer: String,
}

/// Turns a string into a vector of symbols.
///
/// # Example
/// 
/// ```
/// use minilisp::lexer::{Symbol, Literal, Position};
///
/// let program = "(def 'four (+ 2 2))";
/// 
/// let expected = vec![
///     Symbol::LParen(Position::at(1,0)),
///     Symbol::Name(Position::at(1,1), String::from("def")),
///     Symbol::Quote(Position::at(1,5)),
///     Symbol::Name(Position::at(1,6), String::from("four")),
///     Symbol::LParen(Position::at(1,11)),
///     Symbol::Name(Position::at(1,12), String::from("+")),
///     Symbol::Primitive(Position::at(1,14), Literal::Number(2)),
///     Symbol::Primitive(Position::at(1,16), Literal::Number(2)),
///     Symbol::RParen(Position::at(1,17)),
///     Symbol::RParen(Position::at(1,18))
/// ];
///
/// assert_eq!(minilisp::lexer::lex(program), Ok(expected));
/// ```
pub fn lex(input: &str) -> Result<Vec<Symbol>, error::LexerError> {
    let mut buffers = Buffers {
        symbols: Vec::new(),
        buffer: String::new(),
    };

    let seq = input.chars().peekable();
    let mut cursor = tracking::Cursor::new(seq);
    while let Some(x) = cursor.peek() {
        let c = *x;

        if c.is_whitespace() {
            if !buffers.buffer.is_empty() {
                buffers.symbols.push(Symbol::Name(
                    cursor.pos().start_of(buffers.buffer.as_str()),
                    buffers.buffer.clone(),
                ));
                buffers.buffer.clear();
            }
            cursor.next();
            continue;
        }

        if match c {
            // Matching literals
            '"' => push_symbol(&mut buffers, collect_string(&mut cursor)?, &cursor),
            '#' if buffers.buffer.is_empty() => {
                push_symbol(&mut buffers, collect_bool(&mut cursor)?, &cursor)
            }
            n if n.is_ascii_digit() && (buffers.buffer.is_empty() || buffers.buffer == "-") => {
                let number = collect_number(&mut cursor, &mut buffers.buffer);
                push_symbol(&mut buffers, number, &cursor)
            }
            _ => true,
        } {
            match c {
                // Matching Symbols
                '\'' => push_symbol(&mut buffers, Symbol::Quote(cursor.pos()), &cursor),
                '(' => push_symbol(&mut buffers, Symbol::LParen(cursor.pos()), &cursor),
                ')' => push_symbol(&mut buffers, Symbol::RParen(cursor.pos()), &cursor),
                _ => {
                    buffers.buffer.push(c);
                    false // False is returned to match the type
                }
            };
            cursor.next();
        }
    }
    if !buffers.buffer.is_empty() {
        buffers.symbols.push(Symbol::Name(
            cursor.pos().start_of(buffers.buffer.as_str()),
            buffers.buffer.clone(),
        ));
    }
    Ok(buffers.symbols)
}

fn push_symbol(buffers: &mut Buffers, symbol: Symbol, seq: &tracking::Cursor) -> bool {
    if !buffers.buffer.is_empty() {
        buffers.symbols.push(Symbol::Name(
            seq.pos().start_of(buffers.buffer.as_str()),
            buffers.buffer.clone(),
        ));
        buffers.buffer.clear();
    }
    buffers.symbols.push(symbol);
    false
}

fn collect_number(seq: &mut tracking::Cursor, prev: &mut String) -> Symbol {
    let startpos = seq.pos().start_of(prev);
    let mut buffer = prev.clone();
    prev.clear();
    while seq.peek().unwrap_or(&'a').is_ascii_digit() {
        // Using 'a' as a random non digit character
        buffer.push(seq.next().unwrap());
    }
    Symbol::Primitive(startpos, Literal::Number(buffer.parse().unwrap()))
}

fn collect_bool(seq: &mut tracking::Cursor) -> Result<Symbol, error::LexerError> {
    let startpos = seq.pos();
    seq.next();
    match seq.next() {
        Some('t') => Ok(Symbol::Primitive(startpos, Literal::Boolean(true))),
        Some('f') => Ok(Symbol::Primitive(startpos, Literal::Boolean(false))),
        Some(c) => Err(error::LexerError::new(
            startpos,
            format!("Expected #t or #f, found #{}", c).as_str(),
        )),
        None => Err(error::LexerError::new(
            startpos,
            "Expected #t or #f, found EOF",
        )),
    }
}

/// Collects a string litteral based on the following syntax: "(.*(\\")?)*"
fn collect_string(seq: &mut tracking::Cursor) -> Result<Symbol, error::LexerError> {
    let startpos = seq.pos();
    let mut buffer = String::new();
    seq.next();
    loop {
        if match seq.peek() {
            Some(x) => *x,
            None => return Err(error::LexerError::new(seq.pos(), "Expected \", found EOF")),
        } == '\\'
        {
            seq.next();
            buffer.push(match seq.next() {
                Some(x) => x,
                None => {
                    return Err(error::LexerError::new(
                        seq.pos(),
                        "Expected character, found EOF",
                    ))
                }
            });
        }
        match seq.next() {
            Some('"') => break,
            Some(c) => buffer.push(c),
            None => {
                return Err(error::LexerError::new(
                    seq.pos(),
                    "No ending double quote for string!",
                ))
            }
        }
    }
    Ok(Symbol::Primitive(startpos, Literal::String(buffer)))
}
