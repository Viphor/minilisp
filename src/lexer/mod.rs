#[cfg(test)]
mod tests;

pub mod error;
mod tracking;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(i64),
    Boolean(bool),
    String(String),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Quote(tracking::Position),
    LParen(tracking::Position),
    RParen(tracking::Position),
    Name(tracking::Position, String),
    Primitive(tracking::Position, Literal),
}

struct Buffers {
    symbols: Vec<Symbol>,
    buffer: String,
}

pub fn lex(input: &str) -> Result<Vec<Symbol>, error::LexerError> {
    let mut buffers = Buffers {
        symbols: Vec::new(),
        buffer: String::new(),
    };

    let seq = input.chars().peekable();
    let mut cursor = tracking::Cursor::new(seq);
    loop {
        let c = match cursor.peek() {
            Some(x) => *x,
            None => break,
        };

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
            n if n.is_ascii_digit() && buffers.buffer.is_empty() => {
                push_symbol(&mut buffers, collect_number(&mut cursor), &cursor)
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

fn collect_number(seq: &mut tracking::Cursor) -> Symbol {
    let startpos = seq.pos();
    let mut buffer = String::new();
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
