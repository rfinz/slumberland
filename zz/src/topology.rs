use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::ptr::replace;
use cell::{Cell, CellType};
use dimension::Dimension;

pub struct Topology {
    d_cursor: Dimension,
    accursed: Rc<RefCell<Cell>>,
    dimensions: Vec<Dimension>
}

pub struct IterRank {
    rank: Dimension,
    accursed: Rc<RefCell<Cell>>,
    end: bool
}

impl Topology {
    pub fn new(d_cursor: Dimension, cell: Cell) -> Self {
        Topology {
            d_cursor: d_cursor.clone(),
            accursed: Rc::new(RefCell::new(cell)),
            dimensions: vec![d_cursor]
        }
    }

    pub fn iter_rank(&self, rank: Dimension) -> IterRank {
        IterRank {
            rank: rank,
            accursed: Rc::clone(&self.accursed),
            end: false
        }
    }
}

impl Iterator for IterRank {
    type Item = CellType;

    #[inline]
    fn next(&mut self) -> Option<CellType> {
        if self.end {
            return None;
        }

        let ac = (*self.accursed).borrow_mut().clone();
        let res = Some(Cell::as_content(Box::new(ac)));
        {
            let mut cell = (*self.accursed).borrow_mut().clone();

            match cell.get_posward(self.rank.clone()) {
                None => {
                    self.end = true;
                },
                Some(i) => {
                    self.accursed = Rc::clone(&i);
                },
            };
        }

        res

    }
}
