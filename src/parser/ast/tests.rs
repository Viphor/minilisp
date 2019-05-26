use super::*;

#[test]
fn parse_name() {
    let input = vec![Symbol::Name(Position::at(1, 0), String::from("test"))];

    let expected = Ok(Expression::Name(Position::at(1, 0), String::from("test")));

    assert_eq!(expected, Expression::parse(&mut input.iter().peekable()));
}

#[test]
fn parse_primitive() {
    let input = vec![Symbol::Primitive(Position::at(1, 0), Literal::Number(123))];

    let expected = Ok(Expression::Primitive(
        Position::at(1, 0),
        Literal::Number(123),
    ));

    assert_eq!(expected, Expression::parse(&mut input.iter().peekable()));
}

#[test]
fn parse_expression_none() {
    let input = vec![];

    assert_matches!(Expression::parse(&mut input.iter().peekable()), Err(_));
}

#[test]
fn parse_quote() {
    let input = vec![
        Symbol::Quote(Position::at(1, 0)),
        Symbol::Name(Position::at(1, 1), String::from("test")),
    ];

    let expected = Ok(Expression::QuoteExpression(Box::new(Expression::Name(
        Position::at(1, 1),
        String::from("test"),
    ))));

    assert_eq!(expected, Expression::parse(&mut input.iter().peekable()));
}

#[test]
fn parse_compound_none() {
    let input = vec![];

    assert_eq!(
        Ok(Compound::None),
        Compound::parse(&mut input.iter().peekable())
    );
}

#[test]
fn parse_compound() {
    let input = vec![Symbol::Name(Position::at(1, 0), String::from("test"))];

    let expected = Ok(Compound::Some(
        Expression::Name(Position::at(1, 0), String::from("test")),
        Box::new(Compound::None),
    ));

    assert_eq!(expected, Compound::parse(&mut input.iter().peekable()));
}

#[test]
fn parse_list() {
    let input = vec![
        Symbol::LParen(Position::at(1, 0)),
        Symbol::Name(Position::at(1, 1), String::from("test")),
        Symbol::RParen(Position::at(1, 5)),
    ];

    let expected = Ok(Expression::List(List {
        left: Position::at(1, 0),
        content: Box::new(Compound::Some(
            Expression::Name(Position::at(1, 1), String::from("test")),
            Box::new(Compound::None),
        )),
        right: Position::at(1, 5),
    }));

    assert_eq!(expected, Expression::parse(&mut input.iter().peekable()));
}
