#[cfg(test)]
mod tests;

mod tracking;
pub mod error;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(i64),
    Boolean(bool),
    String(String),
    None
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Quote,
    LParen,
    RParen,
    Name(String),
    Primitive(tracking::Position, Literal)
}

pub fn lex(input: &str) -> Result<Vec<Symbol>, error::LexerError> {
    let mut symbols = Vec::new();
    let mut buffer = String::new();

    let seq = input.chars().peekable();
    let mut cursor = tracking::Cursor::new(seq);
    loop {
        let c = match cursor.peek() {
            Some(x) => *x,
            None => break,
        };

        if c.is_whitespace() {
            if !buffer.is_empty() {
                symbols.push(Symbol::Name(buffer.clone()));
                buffer.clear();
            }
            cursor.next();
            continue;
        }
        
        if match c { // Matching literals
            '"' => push_symbol(&mut symbols, collect_string(&mut cursor)?, &mut buffer),
            '#' if buffer.is_empty() =>
                push_symbol(&mut symbols, collect_bool(&mut cursor)?, &mut buffer),
            n if n.is_ascii_digit() && buffer.is_empty() => 
                push_symbol(&mut symbols, collect_number(&mut cursor), &mut buffer),
            _ => true
        } {
            match c { // Matching Symbols
                '\'' => push_symbol(&mut symbols, Symbol::Quote, &mut buffer),
                '(' => push_symbol(&mut symbols, Symbol::LParen, &mut buffer),
                ')' => push_symbol(&mut symbols, Symbol::RParen, &mut buffer),
                _ => { buffer.push(c); false } // False is returned to match the type
            };
            cursor.next();
        }
    };
    if !buffer.is_empty() { symbols.push(Symbol::Name(buffer.clone())); }
    Ok(symbols)
}

fn push_symbol(vec: &mut Vec<Symbol>, symbol: Symbol, buffer: &mut String) -> bool {
    if !buffer.is_empty() {
        vec.push(Symbol::Name(buffer.clone()));
        buffer.clear();
    }
    vec.push(symbol);
    false
}

fn collect_number(seq: &mut tracking::Cursor) -> Symbol {
    let startpos = seq.pos();
    let mut buffer = String::new();
    while seq.peek().unwrap_or(&'a').is_ascii_digit() { // Using 'a' as a random non digit character
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
        Some(c) => Err(error::LexerError::new(startpos, format!("Expected #t or #f, found #{}", c).as_str())),
        None => Err(error::LexerError::new(startpos, "Expected #t or #f, found EOF"))
    }
}

fn collect_string(seq: &mut tracking::Cursor) -> Result<Symbol, error::LexerError> {
    let startpos = seq.pos();
    let mut buffer = String::new();
    seq.next();
    loop {
        if match seq.peek() {
            Some(x) => *x,
            None => return Err(error::LexerError::new(seq.pos(), "Expected \", found EOF"))
        } == '\\' {
            seq.next();
            buffer.push(match seq.next() {
                Some(x) => x,
                None => return Err(error::LexerError::new(seq.pos(), "Expected character, found EOF"))
            });
        }
        match seq.next() {
            Some('"') => break,
            Some(c) => buffer.push(c),
            None => return Err(error::LexerError::new(seq.pos(), "No ending double quote for string!"))
        }
    }
    Ok(Symbol::Primitive(startpos, Literal::String(buffer)))
}
