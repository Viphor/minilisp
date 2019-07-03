use std::rc::Rc;
use super::datastructure::{Construct, Item, Output, Environment};
use super::parser::ast::{self, AST};

#[cfg(test)]
mod tests;

pub fn convert(ast: AST) -> (Vec<usize>, Rc<Environment>) {
    let env = Rc::new(Environment::new());
    let expression_val = convert_compound(*ast.root, env.clone());
    let mut result = Vec::new();

    if let None = expression_val {
        return (result, env)
    }

    let mut expression_list = env.memory.get(expression_val.unwrap());
    while let Some(Output::Data(Item::Construct(li))) = expression_list {
        if let Some(content) = li.car {
            result.push(content);
        }
        if let Some(cdr) = li.cdr {
            expression_list = match env.memory.get(cdr) {
                Some(Output::Data(Item::Construct(_))) => env.memory.get(cdr),
                _ => {
                    result.push(cdr);
                    None
                },
            };
        } else {
            expression_list = None;
        }
    }
    (result, env)
}

fn convert_compound(compound: ast::Compound, mut env: Rc<Environment>) -> Option<usize> {
    match compound {
        ast::Compound::Some(e, c) => {
            let _env = Rc::get_mut(&mut env).unwrap();
            _env.memory.push(Output::Data(Item::Construct(Box::new(Construct::new(
                env.clone(),
                Some(convert_expression(e, env.clone())),
                match convert_compound(*c, env.clone()) {
                    Some(s) => Some(Item::Pointer(s)),
                    None => None,
                },
            )))));
            Some(env.memory.len() - 1)
        },
        ast::Compound::None => None,
    }
}

fn convert_expression(expression: ast::Expression, env: Rc<Environment>) -> Item {
    match expression {
        ast::Expression::QuoteExpression(e) => Item::Construct(Box::new(Construct::new(
            env.clone(),
            Some(Item::Name(String::from("quote"))),
            Some(Item::Construct(Box::new(Construct::new(
                env.clone(),
                Some(convert_expression(*e, env.clone())),
                None,
            )))),
        ))),
        ast::Expression::List(l) => match convert_compound(*l.content, env.clone()) {
            Some(s) => Item::Pointer(s),
            None => Item::Construct(Box::new(Construct::new(env.clone(), None, None))),
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
