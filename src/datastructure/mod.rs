use generational_arena::{Arena, Index};
use std::collections::HashMap;
use std::rc::Rc;

pub type EnvironmentIndex = Index;

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(i64),
    String(String),
    Boolean(bool),
    Name(String),
    Pointer(EnvironmentIndex),
    Construct(Construct),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Construct {
    pub car: Option<EnvironmentIndex>,
    pub cdr: Option<EnvironmentIndex>,
}

impl Construct {
    pub fn new(mut env: Rc<Environment>, car: Option<Item>, cdr: Option<Item>) -> Construct {
        let env = match Rc::get_mut(&mut env) {
            Some(e) => e,
            None => panic!("Could not get write lock for memory."),
        };
        let car = match car {
            Some(Item::Pointer(p)) => Some(p),
            Some(i) => {
                Some(env.alloc(Output::Data(i)))
            }
            None => None,
        };
        let cdr = match cdr {
            Some(Item::Pointer(p)) => Some(p),
            Some(i) => {
                Some(env.alloc(Output::Data(i)))
            }
            None => None,
        };
        Construct { car, cdr }
    }
}

pub enum Output {
    Function(Box<Fn(Output) -> Output>),
    Data(Item),
    None,
}

impl Clone for Output {
    fn clone(&self) -> Self {
        match self {
            Output::Function(_) => panic!("Cannot clone a function."),
            Output::Data(i) => Output::Data(i.clone()),
            Output::None => Output::None,
        }
    }
}

pub struct Environment {
    variables: Vec<HashMap<String, EnvironmentIndex>>,
    memory: Arena<Output>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: vec![HashMap::new()],
            memory: Arena::new(),
        }
    }

    pub fn lookup<T>(&self, key: T) -> Option<&Output>
    where
        T: Into<String>,
    {
        let key = key.into();
        let map = self.variables.iter().rev().find(|&&m| m.contains_key(&key));
        match map {
            Some(m) => self.get(m.get(&key).unwrap().clone()),
            None => None,
        }
    }

    pub fn assign<T>(&mut self, key: T, value: Output)
    where
        T: Into<String>,
    {
        let key = key.into();
        let value = self.alloc(value);
        if let Some(m) = self.variables.last() {
            m.insert(key, value);
        };
    }

    pub fn alloc(&mut self, value: Output) -> EnvironmentIndex {
        self.memory.insert(value)
    }

    pub fn free(&mut self, i: EnvironmentIndex) -> Option<Output> {
        self.memory.remove(i)
    }

    pub fn get(&self, i: EnvironmentIndex) -> Option<&Output> {
        self.memory.get(i)
    }

    pub fn get_mut(&mut self, i: EnvironmentIndex) -> Option<&mut Output> {
        self.memory.get_mut(i)
    }
}
