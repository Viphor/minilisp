use super::*;

pub fn is_number(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::Number(_)) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_number_env() -> EnvItem {
    EnvItem::Function(
        "number?".into(),
        Rc::new(is_number),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_string(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::String(_)) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_string_env() -> EnvItem {
    EnvItem::Function(
        "string?".into(),
        Rc::new(is_string),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_boolean(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::Boolean(_)) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_boolean_env() -> EnvItem {
    EnvItem::Function(
        "boolean?".into(),
        Rc::new(is_boolean),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_name(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::Name(_)) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_name_env() -> EnvItem {
    EnvItem::Function(
        "name?".into(),
        Rc::new(is_name),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_list(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::Cons(_)) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_list_env() -> EnvItem {
    EnvItem::Function(
        "list?".into(),
        Rc::new(is_list),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_none(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Data(Item::None) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_none_env() -> EnvItem {
    EnvItem::Function(
        "none?".into(),
        Rc::new(is_none),
        Parameters::Individual(vec!["value".into()]),
    )
}

pub fn is_function(machine: &mut Machine) -> FunctionOutput {
    let value = machine.lookup("value")?;

    if let EnvItem::Function(_, _, _) = value {
        Ok(EnvItem::Data(Item::Boolean(true)))
    } else {
        Ok(EnvItem::Data(Item::Boolean(false)))
    }
}

pub fn is_function_env() -> EnvItem {
    EnvItem::Function(
        "function?".into(),
        Rc::new(is_function),
        Parameters::Individual(vec!["value".into()]),
    )
}
