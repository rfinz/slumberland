use std::cmp::{PartialEq, Eq};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dimension(String);

impl Dimension {
    pub fn new(rank: String) -> Dimension {
        Dimension(rank)
    }
}
