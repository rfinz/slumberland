use std::cmp::{PartialEq, PartialOrd, Eq};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct Dimension {
    pub name: String
}

impl Dimension {
    pub fn new(name: String) -> Dimension {
        Dimension{ name:name }
    }
}
