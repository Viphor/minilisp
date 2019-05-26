use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

/// Representation of a position in the code.
///
/// NOTE: The line number is 1-indexed, but the character number is 0-indexed.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    /// Denotes the line number (1-indexed)
    line: usize,
    /// Denotes the column number (0-indexed)
    character: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.character)
    }
}

impl Position {
    fn new() -> Position {
        Position {
            line: 1,
            character: 0,
        }
    }
    /// Generates a new `Position` at the given line and character.
    pub fn at(line: usize, character: usize) -> Position {
        Position { line, character }
    }
    /// Returns the line number.
    pub fn line(&self) -> usize {
        self.line
    }
    /// Returns the character number.
    pub fn character(&self) -> usize {
        self.character
    }
    fn new_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
    /// Generates a new `Position` by subtracting the given string from
    /// the current position.
    pub fn start_of(&self, symbol: &str) -> Position {
        Position {
            line: self.line,
            character: self.character - symbol.chars().count(),
        }
    }
}

pub struct Cursor<'a> {
    pos: Position,
    seq: Peekable<Chars<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(seq: Peekable<Chars<'a>>) -> Cursor<'a> {
        Cursor {
            pos: Position::new(),
            seq,
        }
    }
    pub fn pos(&self) -> Position {
        self.pos
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.seq.peek()
    }
    pub fn next(&mut self) -> Option<char> {
        self.pos.character += 1;
        if let Some('\n') = self.peek() {
            self.pos.new_line();
        }
        self.seq.next()
    }
}
