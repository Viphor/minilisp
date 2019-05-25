use minilisp::lexer;

fn main() {
    let test = "(test '(\"cool \\\"string\\\" stuff\" 123))";
    println!("String to test: {}", test);
    println!("{:?}", lexer::lex(test));

    println!("Hello, world!");
}
