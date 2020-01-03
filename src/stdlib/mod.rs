//! This module is meant to be where the whole stdlib should be defined.
//!
//! The main function of the stdlib is the `eval` function.
//! This function has the job of evaluating the data structures into actual code.

pub use super::datastructure::{Cons, EnvItem, Environment, FunctionOutput, Item, Output};
use std::rc::Rc;

pub mod control;
pub mod error;
pub mod eval;
pub mod lambda;
pub mod list;
pub mod math;

pub use eval::eval;
use eval::eval_wrapper;
pub use lambda::lambda;

pub fn stdlib() -> Environment {
    let mut env = Environment::default();

    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    env.assign("eval", EnvItem::Function(Rc::new(eval_wrapper)));
    env.assign("def", EnvItem::Function(Rc::new(def)));
    env.assign("quote", EnvItem::Function(Rc::new(quote)));
    env.assign("+", EnvItem::Function(Rc::new(math::addition)));
    env.assign("add", EnvItem::Function(Rc::new(math::addition)));
    env.assign("-", EnvItem::Function(Rc::new(math::subtraction)));
    env.assign("sub", EnvItem::Function(Rc::new(math::subtraction)));
    env.assign("*", EnvItem::Function(Rc::new(math::multiplication)));
    env.assign("mult", EnvItem::Function(Rc::new(math::multiplication)));
    env.assign("/", EnvItem::Function(Rc::new(math::division)));
    env.assign("div", EnvItem::Function(Rc::new(math::division)));
    env.assign("%", EnvItem::Function(Rc::new(math::modulo)));
    env.assign("mod", EnvItem::Function(Rc::new(math::modulo)));
    env.assign("cons", EnvItem::Function(Rc::new(list::cons)));
    env.assign("if", EnvItem::Function(Rc::new(control::if_control)));
    env.assign("<", EnvItem::Function(Rc::new(math::less_than)));
    env.assign("lt", EnvItem::Function(Rc::new(math::less_than)));
    env.assign(">", EnvItem::Function(Rc::new(math::greater_than)));
    env.assign("gt", EnvItem::Function(Rc::new(math::greater_than)));

    env
}

pub fn def(params: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(c) = params {
        if c.len() > 2 {
            return Err(error::EvalError {
                code: error::EvalErrorCode::E0006,
                message: format!(
                    "Wrong amount of arguments for 'def'. Expexted 2, found {}",
                    c.len()
                ),
            });
        }
        let name;
        if let Item::Name(n) = c.car() {
            name = n;
        } else {
            return Err(error::EvalError {
                code: error::EvalErrorCode::E0009,
                message: format!("'{}' is not a name, and cannot be bound", c.car()),
            });
        }
        if let Item::Cons(cdr) = c.cdr() {
            let res = eval(cdr.car(), env)?;
            env.define(name, res.clone());
            Ok(res)
        } else {
            Err(error::EvalError {
                code: error::EvalErrorCode::E0010,
                message: "Could not parse second parameter".into(),
            })
        }
    } else {
        Err(error::EvalError {
            code: error::EvalErrorCode::E0010,
            message: "Could not parse paramters".into(),
        })
    }
}

pub fn quote(params: &Item, _: &mut Environment) -> FunctionOutput {
    if let Item::Cons(c) = params {
        if let Item::None = c.cdr() {
            Ok(Output::Data(c.car().clone()))
        } else {
            Err(error::EvalError {
                code: error::EvalErrorCode::E0006,
                message: "Only one argument is supported by quote".into(),
            })
        }
    } else {
        Ok(Output::Data(params.clone()))
    }
}

//pub fn list(params: &Item, env: &mut Environment) -> FunctionOutput {
//    let mut
//}
