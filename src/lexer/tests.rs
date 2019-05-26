use super::*;

#[test]
fn lex_string() {
    let out = lex("(test '(new \"cool \\\"string\\\" stuff\") 123 -321 #t #f)");
    assert_eq!(
        vec![
            Symbol::LParen(Position::at(1, 0)),
            Symbol::Name(Position::at(1, 1), String::from("test")),
            Symbol::Quote(Position::at(1, 6)),
            Symbol::LParen(Position::at(1, 7)),
            Symbol::Name(Position::at(1, 8), String::from("new")),
            Symbol::Primitive(
                Position::at(1, 12),
                Literal::String(String::from("cool \"string\" stuff"))
            ),
            Symbol::RParen(Position::at(1, 35)),
            Symbol::Primitive(Position::at(1, 37), Literal::Number(123)),
            Symbol::Primitive(Position::at(1, 41), Literal::Number(-321)),
            Symbol::Primitive(Position::at(1, 46), Literal::Boolean(true)),
            Symbol::Primitive(Position::at(1, 49), Literal::Boolean(false)),
            Symbol::RParen(Position::at(1, 51))
        ],
        out.unwrap()
    );
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
    assert_eq!(
        vec![Symbol::Name(Position::at(1, 0), String::from("name123"))],
        lex("name123").unwrap()
    );
}

#[test]
fn name_with_dash() {
    assert_eq!(
        vec![Symbol::Name(Position::at(1, 0), String::from("test-name"))],
        lex("test-name").unwrap()
    );
}

#[test]
fn number_only() {
    assert_eq!(
        vec![Symbol::Primitive(
            Position::at(1, 0),
            Literal::Number(54321)
        )],
        lex("54321").unwrap()
    );
}

#[test]
fn negative_number() {
    assert_eq!(
        vec![Symbol::Primitive(
            Position::at(1, 0),
            Literal::Number(-12345)
        )],
        lex("-12345").unwrap()
    );
}

#[test]
fn boolean_only_true() {
    assert_eq!(
        vec![Symbol::Primitive(
            Position::at(1, 0),
            Literal::Boolean(true)
        )],
        lex("#t").unwrap()
    );
}

#[test]
fn boolean_only_false() {
    assert_eq!(
        vec![Symbol::Primitive(
            Position::at(1, 0),
            Literal::Boolean(false)
        )],
        lex("#f").unwrap()
    );
}

#[test]
fn boolean_fail() {
    assert_matches!(lex("#j"), Err(_));
}
