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

    pub fn advance_to(&mut self, target: &Term) -> () {
        if target < self {
            let msg = format!(
                "Cannot advance term to a value below the current one! current: {}, target: {}",
                self.0, target.0
            );
            panic!(msg);
        } else {
            self.0 = target.0;
        }
    }
}
