//! This module is meant to be where the whole stdlib should be defined.
//!
//! The main function of the stdlib is the `eval` function.
//! This function has the job of evaluating the data structures into actual code.

pub use super::datastructure::{
    Cons, EnvItem, Environment, FunctionOutput, Item, Number, Output, Parameters,
};
use super::vm;
pub use super::vm::Machine;
use std::rc::Rc;

//pub mod control;
pub mod error;
//pub mod eval;
//pub mod lambda;
pub mod list;
pub mod math;
pub mod types;

//pub use eval::eval;
//pub use lambda::lambda;

pub fn stdlib() -> Environment {
    let mut env = Environment::default();

    env.assign("+", math::addition_env());
    env.assign("add", math::addition_env());
    env.assign("-", math::subtration_env());
    env.assign("sub", math::subtration_env());
    env.assign("*", math::multiplication_env());
    env.assign("mult", math::multiplication_env());
    env.assign("/", math::division_env());
    env.assign("div", math::division_env());
    env.assign("%", math::modulo_env());
    env.assign("mod", math::modulo_env());
    env.assign("<", math::less_than_env());
    env.assign("lt", math::less_than_env());
    env.assign(">", math::greater_than_env());
    env.assign("gt", math::greater_than_env());
    env.assign("=", math::equals_env());
    env.assign("eq", math::equals_env());
    env.assign("cons", list::cons_env());
    env.assign("car", list::car_env());
    env.assign("cdr", list::cdr_env());
    env.assign("number?", types::is_number_env());
    env.assign("string?", types::is_string_env());
    env.assign("boolean?", types::is_boolean_env());
    env.assign("name?", types::is_name_env());
    env.assign("list?", types::is_list_env());
    env.assign("none?", types::is_none_env());
    env.assign("function?", types::is_function_env());

    env
}

pub fn def(machine: &mut Machine) -> FunctionOutput {
    let name_arg = machine.lookup("name")?;
    let value = machine.lookup("value")?;

    let name;
    if let EnvItem::Data(Item::Name(n)) = name_arg {
        name = n;
    } else {
        return Err(error::EvalError::new(
            error::EvalErrorCode::E0009,
            format!("'{:?}' is not a name, and cannot be bound", name_arg),
        ));
    }
    machine.define(name, value.clone())?;
    Ok(value)
}

pub fn def_env() -> EnvItem {
    EnvItem::Function(
        "define".into(),
        Rc::new(def),
        Parameters::Individual(vec!["name".into(), "value".into()]),
    )
}

pub fn quote(machine: &mut Machine) -> FunctionOutput {
    Ok(machine.lookup("value")?)
}
