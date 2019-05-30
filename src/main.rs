use minilisp::{lexer, parser};

fn main() {
    let test = "(test '(\"cool \\\"string\\\" stuff\" 123))";
    println!("String to test: {}", test);
    let mut lexed = lexer::lex(test).unwrap();
    println!("{:?}", lexed);
    let parsed = parser::parse(&mut lexed);
    println!("{:?}", parsed);

    println!("Hello, world!");
}
