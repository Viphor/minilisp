pub use super::datastructure::{Item, Construct};

pub mod types;
pub mod error;

pub fn eval(data: Item, env: &types::Environment) -> Result<types::Output, error::EvalError> {
    match data {
        Item::Construct(list) => eval_function(list, env),
        Item::Name(n) => match env.get(&n) {
            Some(o) => Ok((*o).clone()),
            None => Err(error::EvalError { message: format!("Cannot look up name: '{}'", n) }),
        },
        item => Ok(types::Output::Data(item)),
    }
}

fn eval_function(list: Box<Construct>, env: &types::Environment) -> Result<types::Output, error::EvalError> {
    match list.car {
        Some(Item::Name(s)) => match env.get(&s) {
            Some(types::Output::Function(f)) => Ok(f(eval_cdr(list.cdr, env)?)),
            Some(types::Output::Data(d)) => Ok(eval_data(d.clone(), env)?(eval_cdr(list.cdr, env)?)),
            Some(types::Output::None) => Err(error::EvalError { message: "() is not callable.".to_string() }),
            None => Err(error::EvalError { message: "Name is not bound.".to_string() }),
        },
        Some(Item::Construct(c)) => Ok(eval_data(Item::Construct(c), env)?(eval_cdr(list.cdr, env)?)),
        _ => Err(error:: EvalError { message: "Cannot call non function type.".to_string() })
    }
}

fn eval_cdr(cdr: Option<Item>, env: &types::Environment) -> Result<types::Output, error::EvalError> {
    match cdr {
        Some(i) => eval(i, env),
        None => Ok(types::Output::None),
    }
}

fn eval_data(data: Item, env: &types::Environment) -> Result<Box<Fn(types::Output) -> types::Output>, error::EvalError> {
    match eval(data, env) {
        Ok(types::Output::Function(lambda)) => Ok(lambda),
        Err(e) => Err(e),
        _ => Err(error::EvalError { message: "Cannot call non function type.".to_string() }),
    }
}
