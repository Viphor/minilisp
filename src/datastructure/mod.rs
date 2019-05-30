//use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Item {
    //Function(Box<Fn(Construct) -> Construct>),
    Number(i64),
    String(String),
    Boolean(bool),
    Name(String),
}

//impl PartialEq for Item {
//    fn eq(&self, other: &Self) -> bool {
//        match self {
//            Item::Function(_) => false,
//            Item::Number(n) => if let Item::Number(on) = other {
//                n == on
//            } else { false },
//            Item::String(s) => if let Item::String(os) = other {
//                s == os
//            } else { false },
//            Item::Boolean(b) => if let Item::Boolean(ob) = other {
//                b == ob
//            } else { false },
//            Item::Name(n) => if let Item::Name(on) = other {
//                n == on
//            } else { false },
//        }
//    }
//}
//
//impl fmt::Debug for Item {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            Item::Function(_) => write!(f, "Lambda"),
//            i => write!(f, "{:?}", i),
//        }
//    }
//}

#[derive(Debug, PartialEq)]
pub enum ListItem {
    Item(Item),
    Construct(Box<Construct>),
}

#[derive(Debug, PartialEq)]
pub struct Construct {
    car: Option<ListItem>,
    cdr: Option<ListItem>,
}

impl Construct {
    pub fn new(car: Option<ListItem>, cdr: Option<ListItem>) -> Construct {
        Construct { car, cdr }
    }
    pub fn car(&self) -> &Option<ListItem> {
        &self.car
    }
    pub fn cdr(&self) -> &Option<ListItem> {
        &self.cdr
    }
}
