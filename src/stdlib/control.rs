use super::*;

pub fn if_control(params: &Item, env: &mut Environment) -> FunctionOutput {
    if let Item::Cons(c) = params {
        if c.len() != 3 {
            return Err(error::mismatch_arguments("if", 3, c.len()));
        }
        if let EnvItem::Data(d) = eval(c.car(), env)? {
            if d.into() {
                eval(c.cadr(), env)
            } else {
                eval(c.caddr(), env)
            }
        } else {
            Err(error::unparseable_arguments("if"))
        }
    } else {
        Err(error::unparseable_arguments("if"))
    }
}
