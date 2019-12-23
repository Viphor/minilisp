//! This module is meant to be where the whole stdlib should be defined.
//!
//! The main function of the stdlib is the `eval` function.
//! This function has the job of evaluating the data structures into actual code.

pub use super::datastructure::{Cons, EnvItem, Environment, FunctionOutput, Item, Output};
use std::rc::Rc;

pub mod error;
pub mod eval;
pub mod lambda;
pub mod math;

pub use eval::eval;
pub use lambda::lambda;

pub fn stdlib() -> Environment {
    let mut env = Environment::default();

    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    env.assign("eval", EnvItem::Function(Rc::new(eval)));
    env.assign("def", EnvItem::Function(Rc::new(def)));
    env.assign("quote", EnvItem::Function(Rc::new(quote)));
    env.assign("+", EnvItem::Function(Rc::new(math::addition)));

    env
}

pub fn def(_params: &Item, _env: &mut Environment) -> FunctionOutput {
    Err(error::EvalError {
        code: error::EvalErrorCode::E0007,
        message: "'def' is not yet implemented yet".into(),
    })
}

pub fn quote(params: &Item, _: &mut Environment) -> FunctionOutput {
    if let Item::Cons(c) = params {
        if let Item::None = *c.cdr {
            Ok(Output::Data(*c.car.clone()))
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
