use std::cmp::PartialOrd;

#[derive(Clone, Debug)]
pub struct MonoStack<T> {
    data: Vec<T>,
}

// stores greatest number at top
#[allow(dead_code)]
impl<T> MonoStack<T>
where
    T: PartialOrd,
{
    #[inline]
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn push(&mut self, val: T) {
        while self.data.len() > 0 && self.peek().unwrap() > &val {
            self.data.pop();
        }
        self.data.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
