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

pub fn build_std_env() -> Rc<Environment> {
    let mut env = Environment::default();

    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    env.assign("eval", EnvItem::Function(Rc::new(eval)));

    Rc::new(env)
}
