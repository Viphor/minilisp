use super::*;

pub fn cons(params: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(c) = params {
        if c.len() != 2 {
            return Err(error::mismatch_arguments("cons", 2, c.len()));
        }
        let car_result = eval(c.car(), env)?;
        let car;
        if let EnvItem::Data(d) = car_result {
            car = d;
        } else {
            return Err(error::EvalError {
                code: error::EvalErrorCode::E0010,
                message: "Could not parse the car element".into(),
            });
        }
        let cdr_result = eval(c.cadr(), env)?;
        let cdr;
        if let EnvItem::Data(d) = cdr_result {
            cdr = d;
        } else {
            return Err(error::EvalError {
                code: error::EvalErrorCode::E0010,
                message: "Could not parse the cdr element".into(),
            });
        }
        Ok(EnvItem::Data(Item::Cons(Cons::new(car, cdr))))
    } else {
        Err(error::unparseable_arguments("cons"))
    }
}
