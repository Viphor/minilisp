use super::datastructure::{Cons, Environment, EnvironmentIndex, Item, Output};
use super::parser::ast::{self, AST};
use std::rc::Rc;

//#[cfg(test)]
//mod tests;

pub fn convert(ast: AST) -> (Vec<EnvironmentIndex>, Rc<Environment>) {
    let env = Rc::new(Environment::new());
    let expression_val = convert_compound(*ast.root, env.clone());
    let mut result = Vec::new();

    // TODO Get this to work when iterators are implemented
    //match expression_val {
    //    None => return (result, env),
    //    Some(e) => {
    //        if let Some(Output::Data(Item::Cons(first))) = env.get(e) {
    //            for i in first.clone().iter_ref(env.as_ref()) {
    //                result.push(i.clone());
    //            }
    //        }
    //    }
    //}
    let mut expression_list = env.get(expression_val.unwrap());
    while let Some(Output::Data(Item::Cons(li))) = expression_list {
        if let Some(content) = li.car {
            result.push(content);
        }
        if let Some(cdr) = li.cdr {
            expression_list = match env.get(cdr) {
                Some(Output::Data(Item::Cons(_))) => env.get(cdr),
                _ => {
                    result.push(cdr);
                    None
                }
            };
        } else {
            expression_list = None;
        }
    }
    (result, env)
}

fn convert_compound(compound: ast::Compound, mut env: Rc<Environment>) -> Option<EnvironmentIndex> {
    match compound {
        ast::Compound::Some(e, c) => {
            let env_clone = env.clone();
            let _env = Rc::get_mut(&mut env).unwrap();
            let car = convert_expression(e, env_clone.clone());
            let cdr = convert_compound(*c, env_clone.clone());
            Some(
                _env.alloc(Output::Data(Item::Cons(Cons::from_pointers(
                    car, cdr,
                )))),
            )
        }
        ast::Compound::None => None,
    }
}

fn convert_expression(
    expression: ast::Expression,
    mut env: Rc<Environment>,
) -> Option<EnvironmentIndex> {
    let env_clone = env.clone();
    let _env = Rc::get_mut(&mut env).unwrap();
    match expression {
        ast::Expression::QuoteExpression(e) => {
            let quote = _env.alloc(Output::Data(Item::Name(String::from("quote"))));
            let content = convert_expression(*e, env_clone.clone());
            let wrap = _env.alloc(Output::Data(Item::Cons(Cons::from_pointers(
                content, None,
            ))));
            Some(
                _env.alloc(Output::Data(Item::Cons(Cons::from_pointers(
                    Some(quote),
                    Some(wrap),
                )))),
            )
        }
        ast::Expression::List(l) => convert_compound(*l.content, env.clone()),
        ast::Expression::Name(_, n) => Some(_env.alloc(Output::Data(Item::Name(n)))),
        ast::Expression::Primitive(_, l) => Some(_env.alloc(Output::Data(convert_primitive(l)))),
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
