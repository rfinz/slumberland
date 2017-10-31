use std::cell::RefCell;
use std::rc::Rc;
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

    pub fn insert_posward(&mut self, dimension: Dimension, mut cell: Cell) {
        cell.set_negward(dimension.clone(), Rc::clone(&self.accursed));
        let cell = Rc::new(RefCell::new(cell));
        let mut ac = (*self.accursed).borrow_mut();
        match ac.get_posward(dimension.clone()){
            None => {
                ac.set_posward(dimension, Rc::clone(&cell));
            },
            Some(i) => {
                (*cell).borrow_mut().set_posward(dimension.clone(), Rc::clone(&i));
                (*i).borrow_mut().set_negward(dimension.clone(), Rc::clone(&cell));
                ac.set_posward(dimension.clone(), Rc::clone(&cell));
            }
        }
    }

    pub fn insert_negward(&mut self, dimension: Dimension, mut cell: Cell) {
        cell.set_posward(dimension.clone(), Rc::clone(&self.accursed));
        let cell = Rc::new(RefCell::new(cell));
        let mut ac = (*self.accursed).borrow_mut();
        match ac.get_negward(dimension.clone()){
            None => {
                ac.set_negward(dimension, Rc::clone(&cell));
            },
            Some(i) => {
                (*cell).borrow_mut().set_negward(dimension.clone(), Rc::clone(&i));
                (*i).borrow_mut().set_posward(dimension.clone(), Rc::clone(&cell));
                ac.set_negward(dimension.clone(), Rc::clone(&cell));
            }
        }
    }

}

impl Iterator for IterRank {
    type Item = CellType;

    fn next(&mut self) -> Option<CellType> {
        if self.end {
            return None;
        }

        let ac = (*self.accursed).borrow_mut().clone();
        let res = Some(Cell::as_content(Box::new(ac)));
        {
            let cell = (*self.accursed).borrow_mut().clone();

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

impl DoubleEndedIterator for IterRank {

    fn next_back(&mut self) -> Option<CellType> {
        if self.end {
            return None;
        }

        let ac = (*self.accursed).borrow_mut().clone();
        let res = Some(Cell::as_content(Box::new(ac)));
        {
            let cell = (*self.accursed).borrow_mut().clone();

            match cell.get_negward(self.rank.clone()) {
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
