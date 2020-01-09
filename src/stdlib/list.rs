use super::*;

pub fn cons(_machine: &mut Machine, mut args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        return Err(error::mismatch_arguments("cons", 2, args.len()));
    }

    // We remove the second argument first, in order to not shift it when
    // removing the first argument.
    let second_arg = args.remove(1);
    let first_arg = args.remove(0);

    let car;
    if let EnvItem::Data(d) = first_arg {
        car = d;
    } else {
        return Err(error::EvalError {
            code: error::EvalErrorCode::E0010,
            message: "Could not parse the car element".into(),
        });
    }
    let cdr;
    if let EnvItem::Data(d) = second_arg {
        cdr = d;
    } else {
        return Err(error::EvalError {
            code: error::EvalErrorCode::E0010,
            message: "Could not parse the cdr element".into(),
        });
    }
    Ok(EnvItem::Data(Item::Cons(Cons::new(car, cdr))))
}

//pub fn cons(params: &Item, env: &mut Environment) -> FunctionOutput {
//    if let Item::Cons(c) = params {
//        if c.len() != 2 {
//            return Err(error::mismatch_arguments("cons", 2, c.len()));
//        }
//        let car_result = eval(c.car(), env)?;
//        let car;
//        if let EnvItem::Data(d) = car_result {
//            car = d;
//        } else {
//            return Err(error::EvalError {
//                code: error::EvalErrorCode::E0010,
//                message: "Could not parse the car element".into(),
//            });
//        }
//        let cdr_result = eval(c.cadr(), env)?;
//        let cdr;
//        if let EnvItem::Data(d) = cdr_result {
//            cdr = d;
//        } else {
//            return Err(error::EvalError {
//                code: error::EvalErrorCode::E0010,
//                message: "Could not parse the cdr element".into(),
//            });
//        }
//        Ok(EnvItem::Data(Item::Cons(Cons::new(car, cdr))))
//    } else {
//        Err(error::unparseable_arguments("cons"))
//    }
//}
