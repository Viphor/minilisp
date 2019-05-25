use super::*;

#[test]
fn lex_string() {
    let out = lex(&String::from("(test '(new \"cool \\\"string\\\" stuff\") 123)"));
    assert_eq!(vec![
        Symbol::LParen,
        Symbol::Name(String::from("test")),
        Symbol::Quote,
        Symbol::LParen,
        Symbol::Name(String::from("new")),
        Symbol::Primitive(tracking::Position::at(1,12), Literal::String(String::from("cool \"string\" stuff"))),
        Symbol::RParen,
        Symbol::Primitive(tracking::Position::at(1,37), Literal::Number(123)),
        Symbol::RParen
    ], out.unwrap());
}

#[test]
fn unfinished_string() {
    assert_matches!(lex("\"This should return an error!"), Err(_));
}

#[test]
fn escape_eof() {
    assert_matches!(lex("\"This should return an error!\\"), Err(_));
}

#[test]
fn name_with_number() {
    assert_eq!(vec![Symbol::Name(String::from("name123"))], lex("name123").unwrap());
}

#[test]
fn number_only() {
    assert_eq!(vec![Symbol::Primitive(tracking::Position::at(1,0), Literal::Number(54321))], lex("54321").unwrap());
}
