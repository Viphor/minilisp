use super::*;

fn get_number(item: &EnvItem) -> Result<Number, error::EvalError> {
    match item {
        EnvItem::Data(Item::Number(n)) => Ok(*n),
        _ => Err(error::EvalError::new(
            error::EvalErrorCode::E0010,
            "Addition: Only supports numbers for now",
        )),
    }
}

fn get_args(machine: &mut Machine, args: &str) -> Result<Vec<EnvItem>, error::EvalError> {
    let args = machine.lookup(args)?;
    if let EnvItem::VariableBinding(var) = args {
        Ok(var)
    } else {
        Err(error::unparseable_arguments("addition"))
    }
}

pub fn addition(machine: &mut Machine) -> FunctionOutput {
    let args = get_args(machine, "args")?;
    let mut sum = 0;
    for item in args.iter() {
        sum += get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn addition_env() -> EnvItem {
    EnvItem::Function(
        "addition".into(),
        Rc::new(addition),
        Parameters::All("args".into()),
    )
}

pub fn subtraction(machine: &mut Machine) -> FunctionOutput {
    let args = get_args(machine, "args")?;
    let mut args = args.iter();
    let mut sum = get_number(args.next().unwrap())?;
    for item in args {
        sum -= get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn subtration_env() -> EnvItem {
    EnvItem::Function(
        "subtraction".into(),
        Rc::new(subtraction),
        Parameters::All("args".into()),
    )
}

pub fn multiplication(machine: &mut Machine) -> FunctionOutput {
    let args = get_args(machine, "args")?;
    let mut sum = 1;
    for item in args.iter() {
        sum *= get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn multiplication_env() -> EnvItem {
    EnvItem::Function(
        "multiplication".into(),
        Rc::new(multiplication),
        Parameters::All("args".into()),
    )
}

pub fn division(machine: &mut Machine) -> FunctionOutput {
    let dividend = machine.lookup("dividend")?;
    let divisor = machine.lookup("divisor")?;

    Ok(Output::Data(Item::Number(
        get_number(&dividend)? / get_number(&divisor)?,
    )))
}

pub fn division_env() -> EnvItem {
    EnvItem::Function(
        "division".into(),
        Rc::new(division),
        Parameters::Individual(vec!["dividend".into(), "divisor".into()]),
    )
}

pub fn modulo(machine: &mut Machine) -> FunctionOutput {
    let dividend = machine.lookup("dividend")?;
    let divisor = machine.lookup("divisor")?;

    Ok(Output::Data(Item::Number(
        get_number(&dividend)? % get_number(&divisor)?,
    )))
}

pub fn modulo_env() -> EnvItem {
    EnvItem::Function(
        "modulo".into(),
        Rc::new(modulo),
        Parameters::Individual(vec!["dividend".into(), "divisor".into()]),
    )
}

pub fn less_than(machine: &mut Machine) -> FunctionOutput {
    let left_comparand = machine.lookup("left_comparand")?;
    let right_comparand = machine.lookup("right_comparand")?;

    Ok(Output::Data(Item::Boolean(
        get_number(&left_comparand)? < get_number(&right_comparand)?,
    )))
}

pub fn less_than_env() -> EnvItem {
    EnvItem::Function(
        "less_than".into(),
        Rc::new(less_than),
        Parameters::Individual(vec!["left_comparand".into(), "right_comparand".into()]),
    )
}

pub fn greater_than(machine: &mut Machine) -> FunctionOutput {
    let left_comparand = machine.lookup("left_comparand")?;
    let right_comparand = machine.lookup("right_comparand")?;

    Ok(Output::Data(Item::Boolean(
        get_number(&left_comparand)? > get_number(&right_comparand)?,
    )))
}

pub fn greater_than_env() -> EnvItem {
    EnvItem::Function(
        "greater_than".into(),
        Rc::new(greater_than),
        Parameters::Individual(vec!["left_comparand".into(), "right_comparand".into()]),
    )
}

pub fn equals(machine: &mut Machine) -> FunctionOutput {
    let left_comparand = machine.lookup("left_comparand")?;
    let right_comparand = machine.lookup("right_comparand")?;

    Ok(Output::Data(Item::Boolean(
        left_comparand == right_comparand,
    )))
}

pub fn equals_env() -> EnvItem {
    EnvItem::Function(
        "equals".into(),
        Rc::new(equals),
        Parameters::Individual(vec!["left_comparand".into(), "right_comparand".into()]),
    )
}
