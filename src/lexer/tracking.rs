use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    line: usize,
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
    pub fn at(line: usize, character: usize) -> Position {
        Position { line, character }
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn character(&self) -> usize {
        self.character
    }
    fn new_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
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
