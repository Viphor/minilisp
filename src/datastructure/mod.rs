use super::stdlib::error;
use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(i64),
    String(String),
    Boolean(bool),
    Name(String),
    Cons(Cons),
    None,
}

pub type ConsElement = Item;
type ConsElementContainer<T> = Box<T>;

#[derive(Debug, Clone, PartialEq)]
pub struct Cons {
    pub car: ConsElementContainer<ConsElement>,
    pub cdr: ConsElementContainer<ConsElement>,
}

impl Cons {
    pub fn new(car: ConsElement, cdr: ConsElement) -> Cons {
        Cons {
            car: ConsElementContainer::new(car),
            cdr: ConsElementContainer::new(cdr),
        }
    }
    //    pub fn new(mut env: Rc<Environment>, car: Option<Item>, cdr: Option<Item>) -> Cons {
    //        let env = match Rc::get_mut(&mut env) {
    //            Some(e) => e,
    //            None => panic!("Could not get write lock for memory."),
    //        };
    //        let car = match car {
    //            Some(Item::Pointer(p)) => Some(p),
    //            Some(i) => Some(env.alloc(EnvItem::Data(i))),
    //            None => None,
    //        };
    //        let cdr = match cdr {
    //            Some(Item::Pointer(p)) => Some(p),
    //            Some(i) => Some(env.alloc(EnvItem::Data(i))),
    //            None => None,
    //        };
    //        Cons { car, cdr }
    //    }
    // TODO Fix these iterators
    //pub fn iter<'a>(&self, env: &'a Environment) -> ConsIterator<'a> {
    //    ConsIterator {
    //        env,
    //        current: Some(&self),
    //    }
    //}
    //pub fn iter_ref<'a>(&'a self, env: &'a Environment) -> RefConsIterator {
    //    RefConsIterator {
    //        env,
    //        currentIndex: self.car.as_ref(),
    //    }
    //}
}

impl From<Cons> for Vec<Item> {
    fn from(item: Cons) -> Self {
        let mut vec = Vec::new();
        let mut current = item;
        loop {
            vec.push(current.car.as_ref().clone());
            match current.cdr.as_ref().clone() {
                Item::Cons(c) => current = c,
                Item::None => break,
                i => {
                    vec.push(i);
                    break;
                }
            }
        }
        vec
    }
}

pub type Output = EnvItem;
pub type FunctionOutput = Result<EnvItem, error::EvalError>;
type EnvItemFunction = dyn Fn(&Item, Rc<Environment>) -> FunctionOutput;

#[derive(Clone)]
pub enum EnvItem {
    Function(Rc<EnvItemFunction>),
    Data(Item),
    None,
}

//impl Clone for EnvItem {
//    fn clone(&self) -> Self {
//        match self {
//            EnvItem::Function(_) => panic!("Cannot clone a function."),
//            EnvItem::Data(i) => EnvItem::Data(i.clone()),
//            EnvItem::None => EnvItem::None,
//        }
//    }
//}

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
    parent: Weak<Environment>,
    variables: HashMap<String, EnvItem>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            parent: Weak::new(),
            variables: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn new(parent: Weak<Environment>) -> Environment {
        Environment {
            parent,
            variables: HashMap::new(),
        }
    }

    pub fn rc_new(parent: Rc<Environment>) -> Rc<Environment> {
        Rc::new(Environment {
            parent: Rc::downgrade(&parent),
            variables: HashMap::new(),
        })
    }

    pub fn lookup(&self, key: &str) -> Option<EnvItem> {
        if self.variables.contains_key(key) {
            self.variables.get(key).cloned()
        } else if let Some(out) = self.parent.upgrade() {
            out.lookup(key)
        } else {
            None
        }
    }

    pub fn assign<T>(&mut self, key: T, value: EnvItem) -> Option<EnvItem>
    where
        T: Into<String>,
    {
        let key = key.into();
        self.variables.insert(key, value)
    }
}
