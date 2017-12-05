use std::cmp::{PartialEq, PartialOrd, Eq};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct Dimension(pub String);

impl Dimension {
    pub fn new(rank: String) -> Dimension {
        Dimension(rank)
    }
}
