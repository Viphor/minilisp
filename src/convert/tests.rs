use super::super::lexer::Symbol;
use super::ast::Parsable;
use super::*;

#[test]
fn convert_number() {
    assert_eq!(Item::Number(5), convert_primitive(ast::Literal::Number(5)));
}

#[test]
fn convert_boolean() {
    assert_eq!(
        Item::Boolean(true),
        convert_primitive(ast::Literal::Boolean(true))
    );
}

#[test]
fn convert_string() {
    assert_eq!(
        Item::String(String::from("test")),
        convert_primitive(ast::Literal::String(String::from("test")))
    );
}

#[test]
#[should_panic]
fn convert_none() {
    convert_primitive(ast::Literal::None);
}

#[test]
fn convert_name() {
    assert_eq!(
        Item::Name(String::from("test")),
        convert_expression(ast::Expression::Name(
            ast::Position::at(1, 0),
            String::from("test")
        ))
    );
}

#[test]
fn convert_literal() {
    assert_eq!(
        Item::Number(123),
        convert_expression(ast::Expression::Primitive(
            ast::Position::at(1, 0),
            ast::Literal::Number(123)
        ))
    );
}

#[test]
fn convert_empty_list() {
    let list = ast::List::parse(
        &mut vec![
            Symbol::LParen(ast::Position::at(1, 0)),
            Symbol::RParen(ast::Position::at(1, 1)),
        ]
        .iter()
        .peekable(),
    );
    let res = convert_expression(ast::Expression::List(list.unwrap()));
    assert_eq!(Item::None, res);
}

#[test]
fn convert_quote_expression() {
    //let res = convert_expression(ast::Expression::QuoteExpression(Box::new(
    //    ast::Expression::Name(ast::Position::at(1, 1), String::from("test")),
    //)));
    assert_eq!(
        Item::Cons(Cons::new(
            Item::Name(String::from("quote")),
            Item::Cons(Cons::new(Item::Name(String::from("test")), Item::None))
        )),
        convert_expression(ast::Expression::QuoteExpression(Box::new(
            ast::Expression::Name(ast::Position::at(1, 1), String::from("test"))
        )))
    );
}

#[test]
fn convert_empty_compound() {
    assert_eq!(Item::None, convert_compound(ast::Compound::None));
}

#[test]
fn convert_compound_some() {
    assert_eq!(
        Item::Cons(Cons::new(
            Item::Name(String::from("test")),
            Item::Cons(Cons::new(Item::Number(123), Item::None))
        )),
        convert_compound(ast::Compound::Some(
            ast::Expression::Name(ast::Position::at(1, 0), String::from("test")),
            Box::new(ast::Compound::Some(
                ast::Expression::Primitive(ast::Position::at(2, 0), ast::Literal::Number(123)),
                Box::new(ast::Compound::None)
            ))
        ))
    );
}
