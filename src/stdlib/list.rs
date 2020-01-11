use super::*;

pub fn cons(machine: &mut Machine) -> FunctionOutput {
    let car_arg = machine.lookup("car")?;
    let cdr_arg = machine.lookup("cdr")?;

    let car;
    if let EnvItem::Data(d) = car_arg {
        car = d;
    } else {
        return Err(error::EvalError::new(
            error::EvalErrorCode::E0010,
            "Could not parse the car element",
        ));
    }
    let cdr;
    if let EnvItem::Data(d) = cdr_arg {
        cdr = d;
    } else {
        return Err(error::EvalError::new(
            error::EvalErrorCode::E0010,
            "Could not parse the cdr element",
        ));
    }
    Ok(EnvItem::Data(Item::Cons(Cons::new(car, cdr))))
}

pub fn cons_env() -> EnvItem {
    EnvItem::Function(
        "cons".into(),
        Rc::new(cons),
        Parameters::Individual(vec!["car".into(), "cdr".into()]),
    )
}

pub fn car(machine: &mut Machine) -> FunctionOutput {
    let list = machine.lookup("list")?;

    if let EnvItem::Data(Item::Cons(list)) = list {
        Ok(EnvItem::Data(list.car().clone()))
    } else {
        Err(error::EvalError::new(
            error::EvalErrorCode::E0012,
            "The 'car' function requires a list",
        ))
    }
}

pub fn car_env() -> EnvItem {
    EnvItem::Function(
        "car".into(),
        Rc::new(car),
        Parameters::Individual(vec!["list".into()]),
    )
}

pub fn cdr(machine: &mut Machine) -> FunctionOutput {
    let list = machine.lookup("list")?;

    if let EnvItem::Data(Item::Cons(list)) = list {
        Ok(EnvItem::Data(list.cdr()))
    } else {
        Err(error::EvalError::new(
            error::EvalErrorCode::E0012,
            "The 'cdr' function requires a list",
        ))
    }
}

pub fn cdr_env() -> EnvItem {
    EnvItem::Function(
        "cdr".into(),
        Rc::new(cdr),
        Parameters::Individual(vec!["list".into()]),
    )
}
