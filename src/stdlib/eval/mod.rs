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

fn eval_name(name: &str, env: &Environment) -> Result<Output, error::EvalError> {
    //if let Some(env) = env {
    match env.lookup(name) {
        Some(o) => Ok(o),
        None => Err(error::EvalError {
            code: error::EvalErrorCode::E0002,
            message: format!("Name '{}' is not bound.", name),
        }),
    }
    //} else {
    //    Err(error::EvalError {
    //        code: error::EvalErrorCode::E0003,
    //        message: String::from("Could not load the environment."),
    //    })
    //}
}

fn eval_function(list: &Cons, env: &mut Environment) -> FunctionOutput {
    env.push();
    match list.car.as_ref() {
        Item::Name(s) => match eval_name(s, env)? {
            Output::Function(f) => f(&list.cdr, env),
            _ => Err(error::EvalError {
                code: error::EvalErrorCode::E0004,
                message: format!("Name '{}' is not bound to a function.", s),
            }),
        },
        Item::Cons(_) => {
            if let Output::Function(f) = eval(&list.car, env)? {
                f(&list.cdr, env)
            } else {
                Err(error::EvalError {
                    code: error::EvalErrorCode::E0001,
                    message: format!("'{:?}' cannot evaluate to a function.", list.car),
                })
            }
        }
        //Ok(Output::Function(f)) => Ok(f(*list.cdr, Environment::new(env))),
        //Ok(Output::Data(d)) => Err(error::EvalError {
        //    code: error::EvalErrorCode::E0001,
        //    message: format!("'{:?}' cannot evaluate to a function.", d),
        //}),
        Item::None => Err(error::EvalError {
            code: error::EvalErrorCode::E0001,
            message: String::from("'None' cannot evaluate to a function."),
        }),
        _ => Err(error::EvalError {
            code: error::EvalErrorCode::E0005,
            message: String::from("Could not resolve the reference to car"),
        }),
    }
}
