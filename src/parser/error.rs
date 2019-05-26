use super::super::lexer::Position;

#[derive(Debug, PartialEq)]
pub struct ParserError {
    position: Position,
    message: String,
}

impl ParserError {
    pub fn new(position: Position, message: String) -> ParserError {
        ParserError { position, message }
    }
}
