use syn::{Expr, Ident, LitStr};

#[derive(Debug)]
pub struct Element {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
    pub children: Vec<Child>,
}

#[derive(Debug)]
pub enum Child {
    Text(LitStr),
    Element(Element),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Attribute {
    pub name: Ident,
    pub value: AttributeValue,
}

#[derive(Debug)]
pub enum AttributeValue {
    Str(LitStr),
    Element(Element),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Event {
    pub name: Ident,
    pub expr: Expr,
}
