#[derive(Debug)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &str) -> Person {
        Person { name: name, age: 0 }
    }
}
