pub trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Citation {
    pub author: &'static str,
    pub year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}

// TODO: implement the `min` function used in `main`.
pub fn min<T: LessThan>(a: T, b: T) -> T {
    if a.less_than(&b) {
        a
    } else {
        b
    }
}

