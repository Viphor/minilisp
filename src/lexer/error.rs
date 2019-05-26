use super::tracking::Position;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    position: Position,
    message: String,
}

impl LexerError {
    pub fn new(pos: Position, msg: &str) -> LexerError {
        LexerError {
            position: pos,
            message: String::from(msg),
        }
    }
}
