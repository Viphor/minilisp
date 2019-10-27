use super::*;
use std::rc::Rc;

pub fn lambda(params: &Item, _: Rc<Environment>) -> FunctionOutput {
    let cons;
    if let Item::Cons(c) = params {
        cons = c.clone();
    } else {
        panic!("A lambda expression must have a least a set of arguments and a body");
    }
    let argument_bindings = cons.car;
    let body = cons.cdr.as_ref().clone();

    Ok(Output::Function(Rc::new(move |input, env| {
        let mut env = env.clone();
        variable_binder(argument_bindings.as_ref().clone(), input.clone(), &mut env)?;
        eval(&body, env)
    })))
}

pub fn variable_binder(
    variables: Item,
    values: Item,
    env: &mut Rc<Environment>,
) -> Result<(), error::EvalError> {
    match variables {
        Item::Name(n) => match Rc::get_mut(env) {
            Some(env) => {
                env.assign(n, EnvItem::Data(values));
                Ok(())
            }
            None => Err(error::EvalError {
                code: error::EvalErrorCode::E0003,
                message: "Could not get mutable access to the environment".into(),
            }),
        },
        Item::Cons(c) => {
            let env = match Rc::get_mut(env) {
                Some(e) => e,
                None => {
                    return Err(error::EvalError {
                        code: error::EvalErrorCode::E0003,
                        message: "Could not get mutable access to the environment".into(),
                    });
                }
            };
            let variables: Vec<Item> = c.into();
            if let Item::Cons(v) = values {
                let values: Vec<Item> = v.into();
                if variables.len() != values.len() {
                    return Err(error::EvalError {
                        code: error::EvalErrorCode::E0006,
                        message: format!(
                            "Found {} parameters, expected {}",
                            values.len(),
                            variables.len()
                        ),
                    });
                }
                for (i, var) in variables.iter().enumerate() {
                    match var {
                        Item::Name(n) => env.assign(n, EnvItem::Data(values[i].clone())),
                        e => {
                            return Err(error::EvalError {
                                code: error::EvalErrorCode::E0003,
                                message: format!("'{:?}' is not a valid variable name", e),
                            });
                        }
                    };
                }
            } else if variables.len() > 1 {
                return Err(error::EvalError {
                    code: error::EvalErrorCode::E0006,
                    message: format!("'{:?}'", values),
                });
            };

            Ok(())
        }
        Item::None => {
            if let Item::None = values {
                Ok(())
            } else {
                Err(error::EvalError {
                    code: error::EvalErrorCode::E0006,
                    message: "Too many arguments".into(),
                })
            }
        }
        i => Err(error::EvalError {
            code: error::EvalErrorCode::E0006,
            message: format!("'{:?}' is not a valid parameter name", i),
        }),
    }
}
