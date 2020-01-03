use super::*;

pub fn addition(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!(
                "Addition: Left constituent is not a number. (Only numbers supported for now)"
            ),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(_) => match addition(&list.cdr(), env)? {
                Output::Data(Item::Number(n)) => n,
                _ => panic!(
                    "Addition: Right constituent is not a number. (Only numbers supported for now)"
                ),
            },
            _ => panic!(
                "Addition: Right constituent is not a number. (Only numbers supported for now)"
            ),
        };
        Ok(Output::Data(Item::Number(left + right)))
    } else {
        panic!("Addition: Not a list of numbers");
    }
}

pub fn subtraction(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!(
                "Subtraction: Left constituent is not a number. (Only numbers supported for now)"
            ),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(_) => match addition(&list.cdr(), env)? {
                Output::Data(Item::Number(n)) => n,
                _ => panic!("Subtraction: Right constituent is not a number. (Only numbers supported for now)"),
            },
            _ => panic!("Subtraction: Right constituent is not a number. (Only numbers supported for now)"),
        };
        Ok(Output::Data(Item::Number(left - right)))
    } else {
        panic!("Subtraction: Not a list of numbers");
    }
}

pub fn multiplication(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!("Multiplication: Left constituent is not a number. (Only numbers supported for now)"),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 1i64,
            Item::Cons(_) => match multiplication(&list.cdr(), env)? {
                Output::Data(Item::Number(n)) => n,
                _ => panic!("Multiplication: Right constituent is not a number. (Only numbers supported for now)"),
            },
            _ => panic!("Multiplication: Right constituent is not a number. (Only numbers supported for now)"),
        };
        Ok(Output::Data(Item::Number(left * right)))
    } else {
        panic!("Multiplication: Not a list of numbers");
    }
}

pub fn division(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        if list.len() > 2 {
            panic!("Division: Too many arguments");
        }
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!(
                "Division: Left constituent is not a number. (Only numbers supported for now)"
            ),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 1i64,
            Item::Cons(c) => match eval(c.car(), env)? {
                Output::Data(Item::Number(cn)) => cn,
                _ => panic!(
                    "Division: Right constituent is not a number. (Only numbers supported for now)"
                ),
            },
            _ => panic!(
                "Division: Right constituent is not a number. (Only numbers supported for now)"
            ),
        };
        Ok(Output::Data(Item::Number(left / right)))
    } else {
        panic!("Division: Not a list of numbers");
    }
}

pub fn modulo(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        if list.len() > 2 {
            panic!("Modulo: Too many arguments");
        }
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => {
                panic!("Modulo: Left constituent is not a number. (Only numbers supported for now)")
            }
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(c) => match eval(c.car(), env)? {
                Output::Data(Item::Number(cn)) => cn,
                _ => panic!(
                    "Modulo: Right constituent is not a number. (Only numbers supported for now)"
                ),
            },
            _ => panic!(
                "Modulo: Right constituent is not a number. (Only numbers supported for now)"
            ),
        };
        Ok(Output::Data(Item::Number(left % right)))
    } else {
        panic!("Modulo: Not a list of numbers");
    }
}

pub fn less_than(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        if list.len() > 2 {
            panic!("<: Too many arguments");
        }
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!("<: Left constituent is not a number. (Only numbers supported for now)"),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(c) => match eval(c.car(), env)? {
                Output::Data(Item::Number(cn)) => cn,
                _ => {
                    panic!("<: Right constituent is not a number. (Only numbers supported for now)")
                }
            },
            _ => panic!("<: Right constituent is not a number. (Only numbers supported for now)"),
        };
        Ok(Output::Data(Item::Boolean(left < right)))
    } else {
        panic!("<: Not a list of numbers");
    }
}

pub fn greater_than(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        if list.len() > 2 {
            panic!(">: Too many arguments");
        }
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!(">: Left constituent is not a number. (Only numbers supported for now)"),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(c) => match eval(c.car(), env)? {
                Output::Data(Item::Number(cn)) => cn,
                _ => {
                    panic!(">: Right constituent is not a number. (Only numbers supported for now)")
                }
            },
            _ => panic!(">: Right constituent is not a number. (Only numbers supported for now)"),
        };
        Ok(Output::Data(Item::Boolean(left > right)))
    } else {
        panic!(">: Not a list of numbers");
    }
}
