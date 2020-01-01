use super::*;

pub fn addition(input: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(list) = input {
        let left = match eval(list.car(), env)? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!("Left constituent is not a number. (Only numbers supported for now)"),
        };
        let right = match list.cdr() {
            Item::Number(n) => n,
            Item::None => 0i64,
            Item::Cons(_) => match addition(&list.cdr(), env)? {
                Output::Data(Item::Number(n)) => n,
                _ => panic!("Right constituent is not a number. (Only numbers supported for now)"),
            },
            _ => panic!("Left constituent is not a number. (Only numbers supported for now)"),
        };
        Ok(Output::Data(Item::Number(left + right)))
    } else {
        panic!("Not a list of numbers");
    }
}
