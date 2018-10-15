#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerId(pub usize);

#[derive(Eq, Clone, Copy, Debug, Ord, PartialEq, PartialOrd)]
pub struct Term(usize);

impl Term {
    pub fn new() -> Term {
        Term(0)
    }

    pub fn increment(&mut self) -> () {
        self.0 += 1;
    }

    pub fn update(&mut self, target: &Term) -> () {
        if target > self {
            self.0 = target.0;
        }
    }
}
