use super::*;
use std::rc::Rc;

fn build_environment() -> Environment {
    let mut env = Environment::default();

    env.assign("number", EnvItem::Data(Item::Number(1234)));
    env.assign("+", EnvItem::Function(Rc::new(math::addition)));
    env.assign("lambda", EnvItem::Function(Rc::new(lambda)));

    env
}

#[test]
fn eval_name_in_env() {
    let env = build_environment();

    assert_matches!(
        eval_name("number", &env),
        Ok(Output::Data(Item::Number(1234)))
    );
}

#[test]
fn eval_name_not_in_env() {
    let env = build_environment();

    assert_matches!(
        eval_name("nonsense", &env),
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
        eval_name("nonsense", &Environment::default()),
        Err(error::EvalError {
            code: error::EvalErrorCode::E0003,
            ..
        })
    );
}

#[test]
fn eval_function_with_name() {
    let mut env = build_environment();

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
            &mut env
        ),
        Ok(Output::Data(Item::Number(6)))
    );
}

#[test]
fn eval_value() {
    assert_matches!(
        eval(&Item::Number(42), &mut Environment::default()),
        Ok(Output::Data(Item::Number(42)))
    );
}

#[test]
fn eval_with_name() {
    let mut env = build_environment();

    assert_matches!(
        eval(&Item::Name("number".into()), &mut env),
        Ok(Output::Data(Item::Number(1234)))
    );
}

#[test]
fn eval_cons() {
    let mut env = build_environment();

    assert_matches!(
        eval(
            &Item::Cons(Cons::new(
                Item::Name("+".into()),
                Item::Cons(Cons::new(
                    Item::Number(1),
                    Item::Cons(Cons::new(Item::Number(1), Item::None))
                ))
            )),
            &mut env
        ),
        Ok(Output::Data(Item::Number(2)))
    );
}
