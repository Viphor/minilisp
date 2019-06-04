
use super::datastructure::{Construct, Item, ListItem};
use super::parser::ast::{self, AST};

#[cfg(test)]
mod tests;

pub fn convert(ast: AST) -> Vec<ListItem> {
    let mut expression_list = convert_compound(*ast.root);
    let mut result = Vec::new();

    while let Some(li) = expression_list {
        if let Some(content) = li.car {
            result.push(content);
        }
        if let Some(cdr) = li.cdr {
            expression_list = match cdr {
                ListItem::Item(_) => {
                    result.push(cdr);
                    None
                },
                ListItem::Construct(cons) => Some(cons),
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
                Some(s) => Some(ListItem::Construct(s)),
                None => None,
            },
        ))),
        ast::Compound::None => None,
    }
}

fn convert_expression(expression: ast::Expression) -> ListItem {
    match expression {
        ast::Expression::QuoteExpression(e) => ListItem::Construct(Box::new(Construct::new(
            Some(ListItem::Item(Item::Name(String::from("quote")))),
            Some(ListItem::Construct(Box::new(Construct::new(
                Some(convert_expression(*e)),
                None,
            )))),
        ))),
        ast::Expression::List(l) => match convert_compound(*l.content) {
            Some(s) => ListItem::Construct(s),
            None => ListItem::Construct(Box::new(Construct::new(None, None))),
        },
        ast::Expression::Name(_, n) => ListItem::Item(Item::Name(n)),
        ast::Expression::Primitive(_, l) => ListItem::Item(convert_primitive(l)),
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
