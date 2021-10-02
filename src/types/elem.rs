#[derive(PartialEq, Eq, Debug)]
pub struct Elem {
    pub val: i32,
    pub prior: i32,
}

impl Elem {
    pub fn new(val: i32, prior: i32) -> Self {
        Self { val, prior }
    }
}
