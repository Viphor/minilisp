
use super::datastructure::{Construct, Item};
use super::parser::ast::{self, AST};

#[cfg(test)]
mod tests;

pub fn convert(ast: AST) -> Vec<Item> {
    let mut expression_list = convert_compound(*ast.root);
    let mut result = Vec::new();

    while let Some(li) = expression_list {
        if let Some(content) = li.car {
            result.push(content);
        }
        if let Some(cdr) = li.cdr {
            expression_list = match cdr {
                Item::Construct(cons) => Some(cons),
                _ => {
                    result.push(cdr);
                    None
                },
            };
        } else {
            expression_list = None;
        }
    }
    result
}

fn convert_compound(compound: ast::Compound) -> Option<Box<Construct>> {
    match compound {
        ast::Compound::Some(e, c) => Some(Box::new(Construct::new(
            Some(convert_expression(e)),
            match convert_compound(*c) {
                Some(s) => Some(Item::Construct(s)),
                None => None,
            },
        ))),
        ast::Compound::None => None,
    }
}

fn convert_expression(expression: ast::Expression) -> Item {
    match expression {
        ast::Expression::QuoteExpression(e) => Item::Construct(Box::new(Construct::new(
            Some(Item::Name(String::from("quote"))),
            Some(Item::Construct(Box::new(Construct::new(
                Some(convert_expression(*e)),
                None,
            )))),
        ))),
        ast::Expression::List(l) => match convert_compound(*l.content) {
            Some(s) => Item::Construct(s),
            None => Item::Construct(Box::new(Construct::new(None, None))),
        },
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
