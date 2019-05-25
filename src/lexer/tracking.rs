use std::str::Chars;
use std::iter::Peekable;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    line: u64,
    character: u64
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
            character: 0
        }
    }
    pub fn at(line: u64, character: u64) -> Position {
        Position { line, character }
    }
    pub fn line(&self) -> u64 {
        self.line
    }
    pub fn character(&self) -> u64 {
        self.character
    }
    fn new_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
}

pub struct Cursor<'a> {
    pos: Position,
    seq: Peekable<Chars<'a>>
}

impl<'a> Cursor<'a> {
    pub fn new(seq: Peekable<Chars<'a>>) -> Cursor<'a> {
        Cursor {
            pos: Position::new(),
            seq: seq
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
