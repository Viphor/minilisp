use super::stdlib::error;
use super::vm::Machine;
use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::rc::Rc;
use std::slice::Iter;

pub type Number = i64;

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(Number),
    String(String),
    Boolean(bool),
    Name(String),
    Cons(Cons),
    None,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Number(num) => write!(f, "{}", num),
            Item::String(s) => write!(f, "\"{}\"", s),
            Item::Boolean(b) => write!(f, "{}", b),
            Item::Name(n) => write!(f, "{}", n),
            Item::Cons(c) => write!(f, "{}", c),
            Item::None => write!(f, "()"),
        }
    }
}

impl From<Item> for bool {
    fn from(item: Item) -> Self {
        match item {
            Item::Number(num) => num != 0,
            Item::String(s) => !s.is_empty(),
            Item::Boolean(b) => b,
            Item::Name(_) => true,
            Item::Cons(c) => !c.is_empty(),
            Item::None => false,
        }
    }
}

pub type ConsElement = Item;

#[derive(Debug, Clone, PartialEq)]
pub struct Cons {
    data: Vec<ConsElement>,
    is_null_terminated: bool,
}

impl Default for Cons {
    fn default() -> Self {
        Cons {
            data: vec![ConsElement::None],
            is_null_terminated: true,
        }
    }
}

impl Cons {
    pub fn new(car: ConsElement, cdr: ConsElement) -> Cons {
        let mut data = vec![car];
        let is_null_terminated;
        match cdr {
            ConsElement::Cons(cons) => {
                data.extend(cons.data.iter().cloned());
                is_null_terminated = cons.is_null_terminated;
            }
            ConsElement::None => {
                is_null_terminated = true;
            }
            _ => {
                data.push(cdr);
                is_null_terminated = false;
            }
        }

        Cons {
            data,
            is_null_terminated,
        }
    }

    pub fn iter(&self) -> Iter<Item> {
        self.data.iter()
    }

    pub fn car(&self) -> &ConsElement {
        &self.data[0]
    }

    pub fn cdr(&self) -> ConsElement {
        match self.data.len() {
            2 if !self.is_null_terminated => self.data[1].clone(),
            1 => ConsElement::None,
            0 => panic!("A Cons should never be empty! Contact your vendor. this is a bug"),
            _ => ConsElement::Cons(Cons {
                data: (&self.data[1..]).to_vec(),
                is_null_terminated: self.is_null_terminated,
            }),
        }
    }

    pub fn cadr(&self) -> &ConsElement {
        if self.len() > 2 || (self.len() > 1 && self.is_null_terminated) {
            &self.data[1]
        } else {
            panic!("Not enough elements!"); // TODO This should be handled more gracefully
        }
    }

    pub fn caddr(&self) -> &ConsElement {
        if self.len() > 3 || (self.len() > 2 && self.is_null_terminated) {
            &self.data[2]
        } else {
            panic!("Not enough elements!"); // TODO This should be handled more gracefully
        }
    }

    pub fn cadddr(&self) -> &ConsElement {
        if self.len() > 4 || (self.len() > 3 && self.is_null_terminated) {
            &self.data[3]
        } else {
            panic!("Not enough elements!"); // TODO This should be handled more gracefully
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 1 && self.data[0] == ConsElement::None
    }

    pub fn map<F>(self, f: F) -> Cons
    where
        F: FnMut(&ConsElement) -> ConsElement,
    {
        Cons {
            data: self.data.iter().map(f).collect(),
            is_null_terminated: self.is_null_terminated,
        }
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for it in self.iter().take(1) {
            write!(f, "{}", it)?;
        }
        if self.len() > 2 {
            for it in self.iter().skip(1).take(self.len() - 2) {
                write!(f, " {}", it)?;
            }
        }
        if !self.is_null_terminated {
            write!(f, " .")?;
        }
        if self.len() > 1 {
            for it in self.iter().skip(self.len() - 1) {
                write!(f, " {}", it)?;
            }
        }
        write!(f, ")")
    }
}

impl From<Cons> for Vec<Item> {
    fn from(item: Cons) -> Self {
        item.data
    }
}

impl From<Vec<Item>> for Cons {
    fn from(list: Vec<Item>) -> Self {
        Cons {
            data: list,
            is_null_terminated: true,
        }
    }
}

pub type Output = EnvItem;
pub type FunctionOutput = Result<EnvItem, error::EvalError>;
pub type EnvItemFunction = dyn Fn(&mut Machine, Vec<EnvItem>) -> FunctionOutput;
pub type EnvItemFunctionWrapped = Rc<EnvItemFunction>;

#[derive(Clone)]
pub enum EnvItem {
    Function(EnvItemFunctionWrapped),
    Data(Item),
    None,
}

impl fmt::Debug for EnvItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvItem::Function(fun) => write!(f, "<Function@{:p}>", fun),
            EnvItem::Data(d) => write!(f, "{:?}", d),
            EnvItem::None => write!(f, "None"),
        }
    }
}

impl PartialEq for EnvItem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            EnvItem::Function(f) => {
                if let EnvItem::Function(o) = other {
                    let left: *const EnvItemFunction = f.as_ref();
                    let right: *const EnvItemFunction = o.as_ref();
                    left == right
                } else {
                    false
                }
            }
            EnvItem::Data(d) => {
                if let EnvItem::Data(o) = other {
                    d == o
                } else {
                    false
                }
            }
            EnvItem::None => &EnvItem::None == other,
        }
    }
}

pub struct Environment {
    variables: Vec<HashMap<String, EnvItem>>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            variables: vec![HashMap::new()],
        }
    }
}

impl Environment {
    /// Pushes a new environment layer on top of the environment stack
    pub fn push(&mut self) {
        self.variables.push(HashMap::new());
    }

    /// Pops the top layer of the environment stack
    pub fn pop(&mut self) {
        if self.variables.len() > 1 {
            self.variables.pop();
        }
    }

    /// Used for looking up a named value within the environment.
    /// Starting from the top layer, which shadows the lower layers, and moves
    /// down the stack. If no item is found, None is returned.
    pub fn lookup(&self, key: &str) -> EnvItem {
        for var in self.variables.iter().rev() {
            if var.contains_key(key) {
                return match var.get(key) {
                    Some(item) => item.clone(),
                    None => EnvItem::None,
                };
            }
        }
        EnvItem::None
    }

    /// This assigns a value to the key in the top layer of the environment
    /// stack.
    pub fn assign<T>(&mut self, key: T, value: EnvItem) -> Option<EnvItem>
    where
        T: Into<String>,
    {
        let key = key.into();
        if let Some(var) = self.variables.last_mut() {
            var.insert(key, value)
        } else {
            None
        }
    }

    /// This assigns a value to the key in the bottom layer of the environment
    /// stack. This is called define because this is defining a value for the
    /// current runtime and not just the stack frame.
    pub fn define<T>(&mut self, key: T, value: EnvItem) -> Option<EnvItem>
    where
        T: Into<String>,
    {
        let key = key.into();
        if let Some(var) = self.variables.first_mut() {
            println!("mark!");
            let res = var.insert(key, value);
            println!("{:?}", res);
            res
        } else {
            None
        }
    }
}
