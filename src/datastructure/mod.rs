use super::stdlib::error;
use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(i64),
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

pub type ConsElement = Item;
//type ConsElementContainer<T> = Box<T>;

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

//pub struct Cons {
//    pub car: ConsElementContainer<ConsElement>,
//    pub cdr: ConsElementContainer<ConsElement>,
//}

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

        //Cons {
        //    car: ConsElementContainer::new(car),
        //    cdr: ConsElementContainer::new(cdr),
        //}
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

    //pub fn iter(&self) -> ConsIter {
    //    ConsIter {
    //        current: Some(self),
    //        special_case: None,
    //    }
    //}

    //pub fn map<F>(&self, fun: F) -> Cons
    //where
    //    F: Fn(&ConsElement) -> ConsElement,
    //{
    //    let mut base = Cons::new(Item::None, Item::None);
    //    let mut current = &mut base;
    //    for elem in self.iter() {
    //        current.car = ConsElementContainer::new(fun(elem));
    //        current.cdr = ConsElementContainer::new(Item::Cons(Cons::new(Item::None, Item::None)));
    //        if let Item::Cons(cdr) = current.cdr.as_mut() {
    //            current = cdr;
    //        };
    //    }
    //    current.cdr = ConsElementContainer::new(Item::None);
    //    base
    //}
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
        //let mut vec = Vec::new();
        //let mut current = item;
        //loop {
        //    vec.push(current.car.as_ref().clone());
        //    match current.cdr.as_ref().clone() {
        //        Item::Cons(c) => current = c,
        //        Item::None => break,
        //        i => {
        //            vec.push(i);
        //            break;
        //        }
        //    }
        //}
        //vec
    }
}

//pub struct ConsIter<'a> {
//    current: Option<&'a Cons>,
//    special_case: Option<&'a Item>,
//}
//
//impl<'a> Iterator for ConsIter<'a> {
//    type Item = &'a Item;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        match self.current {
//            Some(c) => {
//                let res = &c.car;
//                self.current = match &*c.cdr {
//                    Item::Cons(cc) => Some(cc),
//                    Item::None => None,
//                    i => {
//                        self.special_case = Some(i);
//                        None
//                    }
//                };
//                Some(res)
//            }
//            None => match self.special_case {
//                Some(Item::None) => {
//                    self.special_case = None;
//                    None
//                }
//                Some(i) => {
//                    self.special_case = None;
//                    Some(i)
//                }
//                None => None,
//            },
//        }
//    }
//}

pub type Output = EnvItem;
pub type FunctionOutput = Result<EnvItem, error::EvalError>;
type EnvItemFunction = dyn Fn(&Item, &mut Environment) -> FunctionOutput;

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
    pub fn push(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.variables.pop();
    }

    //pub fn rc_new(parent: Rc<Environment>) -> Rc<Environment> {
    //    Rc::new(Environment {
    //        parent: Rc::downgrade(&parent),
    //        variables: HashMap::new(),
    //    })
    //}

    pub fn lookup(&self, key: &str) -> Option<EnvItem> {
        for var in self.variables.iter().rev() {
            if var.contains_key(key) {
                return var.get(key).cloned();
            }
        }
        None
    }

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
}
