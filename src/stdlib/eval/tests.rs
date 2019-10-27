use super::*;
use std::rc::Rc;

fn addition(input: &Item, env: Rc<Environment>) -> FunctionOutput {
    if let Item::Cons(list) = input {
        let left = match eval(&list.car, env.clone())? {
            Output::Data(Item::Number(n)) => n,
            Output::Data(Item::None) => 0,
            _ => panic!("Left constituent is not a number. (Only numbers supported for now)"),
        };
        let right = match list.cdr.as_ref() {
            Item::Number(n) => *n,
            Item::None => 0i64,
            Item::Cons(_) => match addition(&list.cdr, env)? {
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

fn build_environment() -> Rc<Environment> {
    let mut env = Environment::default();

    env.assign("number", EnvItem::Data(Item::Number(1234)));
    env.assign("+", EnvItem::Function(Rc::new(addition)));
    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));
    Rc::new(env)
}

#[test]
fn eval_name_in_env() {
    let env = build_environment();

    assert_matches!(
        eval_name("number", env),
        Ok(Output::Data(Item::Number(1234)))
    );
}

#[test]
fn eval_name_not_in_env() {
    let env = build_environment();

    assert_matches!(
        eval_name("nonsense", env),
        Err(error::EvalError {
            code: error::EvalErrorCode::E0002,
            ..
        })
    );
}

#[test]
#[ignore]
fn eval_name_no_env() {
    assert_matches!(
        eval_name("nonsense", Rc::new(Environment::default())),
        Err(error::EvalError {
            code: error::EvalErrorCode::E0003,
            ..
        })
    );
}

#[test]
fn eval_function_with_name() {
    let env = build_environment();

    assert_matches!(
        eval_function(
            &Cons::new(
                Item::Name("+".to_string()),
                Item::Cons(Cons::new(
                    Item::Number(1),
                    Item::Cons(Cons::new(
                        Item::Number(2),
                        Item::Cons(Cons::new(Item::Number(3), Item::None))
                    ))
                ))
            ),
            env
        ),
        Ok(Output::Data(Item::Number(6)))
    );
}

#[test]
fn eval_value() {
    assert_matches!(
        eval(&Item::Number(42), Rc::new(Environment::default())),
        Ok(Output::Data(Item::Number(42)))
    );
}

#[test]
fn eval_with_name() {
    let env = build_environment();

    assert_matches!(
        eval(&Item::Name("number".into()), env),
        Ok(Output::Data(Item::Number(1234)))
    );
}

#[test]
fn eval_cons() {
    let env = build_environment();

    assert_matches!(
        eval(
            &Item::Cons(Cons::new(
                Item::Name("+".into()),
                Item::Cons(Cons::new(
                    Item::Number(1),
                    Item::Cons(Cons::new(Item::Number(1), Item::None))
                ))
            )),
            env
        ),
        Ok(Output::Data(Item::Number(2)))
    );
}
