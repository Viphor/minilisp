use std::collections::HashMap;

use super::*;


pub enum Output {
    Function(Box<Fn(Output) -> Output>),
    Data(Item),
    None
}

impl Clone for Output {
    fn clone(&self) -> Self {
        match self {
            Output::Function(_) => panic!("Cannot clone a function."),
            Output::Data(i) => Output::Data(i.clone()),
            Output::None => Output::None
        }
    }
}

pub type Environment = HashMap<String, Output>;
