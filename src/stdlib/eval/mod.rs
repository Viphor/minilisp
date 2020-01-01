use super::*;

#[cfg(test)]
mod tests;

/// This function work by first looking at the type of the `Item` that it receives.
///
/// If the type is a `Cons` then it will evaluate this as a function call.
/// If the type is a `Name` then it will lookup the contents of the name, without
/// evaluating the contents.
/// Otherwise it will evaluate the item to the value.
///
/// When evaluating a function call, then it will take the `car` of the `Cons` as
/// the function, either by looking up the function in the environment
/// if the `car` is a name, or evaluate the item to a function, otherwise throw
/// an `EvalError`.
///
/// The `cdr` will not be evaluated. That is up to the called function to do.
pub fn eval(data: &Item, env: &mut Environment) -> Result<Output, error::EvalError> {
    match data {
        Item::Cons(list) => eval_function(list, env),
        Item::Name(n) => eval_name(&n, &env),
        item => Ok(Output::Data(item.clone())),
    }
}

/// This method is the one used within the language, which will first evaluate
/// all the parameters, as this is the convention that all functions should do
/// themselves.
pub fn eval_wrapper(data: &Item, env: &mut Environment) -> FunctionOutput {
    let param;
    if let Item::Cons(cons) = data {
        if cons.len() > 1 {
            return Err(error::EvalError {
                code: error::EvalErrorCode::E0006,
                message: format!("Too many arguments. Expected: 1, found: {}", cons.len()),
            });
        } else {
            param = eval(cons.car(), env)?;
        }
    } else {
        param = eval(data, env)?;
    }
    match param {
        EnvItem::Data(d) => eval(&d, env),
        EnvItem::Function(_) => Err(error::EvalError {
            code: error::EvalErrorCode::E0008,
            message: "Function could not be evaluated".into(),
        }),
        EnvItem::None => Ok(Output::None),
    }
}

fn eval_name(name: &str, env: &Environment) -> Result<Output, error::EvalError> {
    match env.lookup(name) {
        Some(o) => Ok(o),
        None => Err(error::EvalError {
            code: error::EvalErrorCode::E0002,
            message: format!("Name '{}' is not bound.", name),
        }),
    }
}

fn eval_function(list: &Cons, env: &mut Environment) -> FunctionOutput {
    env.push();
    let res = match list.car() {
        Item::Name(s) => match eval_name(s, env)? {
            Output::Function(f) => f(&list.cdr(), env),
            _ => Err(error::EvalError {
                code: error::EvalErrorCode::E0004,
                message: format!("Name '{}' is not bound to a function.", s),
            }),
        },
        Item::Cons(_) => {
            //match eval(&list.car, env)? {
            //
            //}
            if let Output::Function(f) = eval(list.car(), env)? {
                f(&list.cdr(), env)
            } else {
                Err(error::EvalError {
                    code: error::EvalErrorCode::E0001,
                    message: format!("'{}' cannot evaluate to a function.", list.car()),
                })
            }
        }
        Item::None => Err(error::EvalError {
            code: error::EvalErrorCode::E0001,
            message: String::from("'None' cannot evaluate to a function."),
        }),
        _ => Err(error::EvalError {
            code: error::EvalErrorCode::E0005,
            message: String::from("Could not resolve the reference to car"),
        }),
    };
    env.pop();
    res
}
