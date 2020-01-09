use super::*;

fn get_number(item: &EnvItem) -> Result<Number, error::EvalError> {
    match item {
        EnvItem::Data(Item::Number(n)) => Ok(*n),
        _ => Err(error::EvalError {
            message: "Addition: Only supports numbers for now".into(),
            code: error::EvalErrorCode::E0010,
        }),
    }
}

pub fn addition(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    let mut sum = 0;
    for item in args.iter() {
        sum += get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn subtraction(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    if args.is_empty() {
        return Err(error::EvalError {
            message: "Subtraction: No arguments supplied".into(),
            code: error::EvalErrorCode::E0010,
        });
    }
    let mut args = args.iter();
    let mut sum = get_number(args.next().unwrap())?;
    for item in args {
        sum -= get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn multiplication(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    let mut sum = 1;
    for item in args.iter() {
        sum *= get_number(item)?;
    }
    Ok(Output::Data(Item::Number(sum)))
}

pub fn division(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        Err(error::mismatch_arguments("division", 2, args.len()))
    } else {
        Ok(Output::Data(Item::Number(
            get_number(&args[0])? / get_number(&args[1])?,
        )))
    }
}

pub fn modulo(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        Err(error::mismatch_arguments("modulo", 2, args.len()))
    } else {
        Ok(Output::Data(Item::Number(
            get_number(&args[0])? % get_number(&args[1])?,
        )))
    }
}

pub fn less_than(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        Err(error::mismatch_arguments("less than", 2, args.len()))
    } else {
        Ok(Output::Data(Item::Boolean(
            get_number(&args[0])? < get_number(&args[1])?,
        )))
    }
}

pub fn greater_than(_machine: &mut Machine, args: Vec<EnvItem>) -> FunctionOutput {
    if args.len() != 2 {
        Err(error::mismatch_arguments("greater than", 2, args.len()))
    } else {
        Ok(Output::Data(Item::Boolean(
            get_number(&args[0])? > get_number(&args[1])?,
        )))
    }
}
