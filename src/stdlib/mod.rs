//! This module is meant to be where the whole stdlib should be defined.
//!
//! The main function of the stdlib is the `eval` function.
//! This function has the job of evaluating the data structures into actual code.

pub use super::datastructure::{Cons, EnvItem, Environment, FunctionOutput, Item, Number, Output};
use super::vm;
pub use super::vm::Machine;
use std::rc::Rc;

//pub mod control;
pub mod error;
//pub mod eval;
pub mod lambda;
pub mod list;
pub mod math;

//pub use eval::eval;
pub use lambda::lambda;

pub fn stdlib() -> Environment {
    let mut env = Environment::default();

    //env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    //env.assign("eval", EnvItem::Function(Rc::new(eval_wrapper)));
    //env.assign("def", EnvItem::Function(Rc::new(def)));
    //env.assign("define", EnvItem::Function(Rc::new(def)));
    //env.assign("quote", EnvItem::Function(Rc::new(quote)));
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
    //env.assign("if", EnvItem::Function(Rc::new(control::if_control)));
    env.assign("<", EnvItem::Function(Rc::new(math::less_than)));
    env.assign("lt", EnvItem::Function(Rc::new(math::less_than)));
    env.assign(">", EnvItem::Function(Rc::new(math::greater_than)));
    env.assign("gt", EnvItem::Function(Rc::new(math::greater_than)));

    env
}

pub fn def(machine: &mut Machine, mut args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        return Err(error::mismatch_arguments("def", 2, args.len()));
    }

    // We remove the second argument first, in order to not shift it when
    // removing the first argument.
    let second_arg = args.remove(1);
    let first_arg = args.remove(0);

    let name;
    if let EnvItem::Data(Item::Name(n)) = first_arg {
        name = n;
    } else {
        return Err(error::EvalError {
            code: error::EvalErrorCode::E0009,
            message: format!("'{:?}' is not a name, and cannot be bound", first_arg),
        });
    }
    machine.define(name, second_arg.clone())?;
    Ok(second_arg)
}

pub fn quote(_machine: &mut Machine, mut args: Vec<EnvItem>) -> FunctionOutput {
    Ok(args.remove(0))
}
