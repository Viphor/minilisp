//! This module is meant to be where the whole stdlib should be defined.
//!
//! The main function of the stdlib is the `eval` function.
//! This function has the job of evaluating the data structures into actual code.

pub use super::datastructure::{Cons, EnvItem, Environment, FunctionOutput, Item, Output};
use std::rc::Rc;

pub mod error;
pub mod eval;
pub mod lambda;

pub use eval::eval;
pub use lambda::lambda;

pub fn stdlib() -> Rc<Environment> {
    let mut env = Environment::default();

    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    env.assign("eval", EnvItem::Function(Rc::new(eval)));
    env.assign("def", EnvItem::Function(Rc::new(def)));

    Rc::new(env)
}

pub fn def(_params: &Item, _env: Rc<Environment>) -> FunctionOutput {
    Err(error::EvalError {
        code: error::EvalErrorCode::E0007,
        message: "'def' is not yet implemented yet".into(),
    })
}
