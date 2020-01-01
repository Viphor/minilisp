use super::*;
use std::rc::Rc;

pub fn lambda(params: &Item, _: &mut Environment) -> FunctionOutput {
    let cons;
    if let Item::Cons(c) = params {
        cons = c.clone();
    } else {
        panic!("A lambda expression must have a least a set of arguments and a body");
    }
    let argument_bindings = cons.car().clone();
    let body = match cons.cdr() {
        Item::Cons(c) => c.into(),
        i => vec![i],
    };

    Ok(Output::Function(Rc::new(move |input, mut env| {
        variable_binder(argument_bindings.clone(), input.clone(), &mut env)?;

        let mut last_res = EnvItem::Data(Item::None);
        for statement in body.iter() {
            last_res = eval(&statement, &mut env)?;
        }
        Ok(last_res)
    })))
}

pub fn variable_binder(
    variables: Item,
    values: Item,
    env: &mut Environment,
) -> Result<(), error::EvalError> {
    match variables {
        Item::Name(n) => {
            let val = eval(&values, env)?;
            env.assign(n, val);
            Ok(())
        }
        Item::Cons(c) => {
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
                        Item::Name(n) => {
                            let val = eval(&values[i], env)?;
                            env.assign(n, val)
                        }
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
