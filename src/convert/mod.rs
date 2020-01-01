use super::datastructure::{Cons, Item};
use super::parser::ast::{self, AST};

#[cfg(test)]
mod tests;

pub fn convert(ast: AST) -> Vec<Item> {
    let expression_val = convert_compound(*ast.root);

    if let Item::Cons(result) = expression_val {
        result.into()
    } else {
        vec![expression_val]
    }
}

fn convert_compound(compound: ast::Compound) -> Item {
    match compound {
        ast::Compound::Some(e, c) => {
            let car = convert_expression(e);
            let cdr = convert_compound(*c);
            Item::Cons(Cons::new(car, cdr))
        }
        ast::Compound::None => Item::None,
    }
}

fn convert_expression(expression: ast::Expression) -> Item {
    match expression {
        ast::Expression::QuoteExpression(e) => {
            let quote = Item::Name(String::from("quote"));
            let content = convert_expression(*e);
            let wrap = Item::Cons(Cons::new(content, Item::None));
            Item::Cons(Cons::new(quote, wrap))
        }
        ast::Expression::List(l) => convert_compound(*l.content),
        ast::Expression::Name(_, n) => Item::Name(n),
        ast::Expression::Primitive(_, l) => convert_primitive(l),
    }
}

fn convert_primitive(primitive: ast::Literal) -> Item {
    match primitive {
        ast::Literal::Number(n) => Item::Number(n),
        ast::Literal::Boolean(b) => Item::Boolean(b),
        ast::Literal::String(s) => Item::String(s),
        ast::Literal::None => panic!("This literal type is not yet supported!"),
    }
}
