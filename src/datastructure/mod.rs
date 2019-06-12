#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(i64),
    String(String),
    Boolean(bool),
    Name(String),
    Construct(Box<Construct>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Construct {
    pub car: Option<Item>,
    pub cdr: Option<Item>,
}

impl Construct {
    pub fn new(car: Option<Item>, cdr: Option<Item>) -> Construct {
        Construct { car, cdr }
    }
}
