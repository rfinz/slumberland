use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::{Cell, CellType};
use crate::dimension::Dimension;

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

    pub fn get_accursed(&self) {
        Rc::clone(&self.accursed);
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

    pub fn link(dimension: Dimension, negward: Rc<RefCell<Cell>>, posward: Rc<RefCell<Cell>> ) {
        (*negward).borrow_mut().set_posward(dimension.clone(), Rc::clone(&posward));
        (*posward).borrow_mut().set_negward(dimension.clone(), Rc::clone(&negward));
    }

    pub fn unlink_posward(dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        match (*cell).borrow_mut().unlink_posward(dimension.clone()){
            None => (),
            Some(i) => {
              (*i).borrow_mut().unlink_negward(dimension.clone());
            }
        };
    }

    pub fn unlink_negward(dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        match (*cell).borrow_mut().unlink_negward(dimension.clone()){
            None => (),
            Some(i) => {
              (*i).borrow_mut().unlink_posward(dimension.clone());
            }
        };
    }

    pub fn accurse_posward(&mut self) {
        let curr = (*self.accursed).borrow_mut().clone();

        match curr.get_posward(self.d_cursor.clone()){
            None => (),
            Some(i) => {
                self.accursed = Rc::clone(&i)
            }
        }
    }

    pub fn accurse_negward(&mut self) {
        let curr = (*self.accursed).borrow_mut().clone();

        match curr.get_negward(self.d_cursor.clone()){
            None => (),
            Some(i) => {
                self.accursed = Rc::clone(&i)
            }
        }
    }

    pub fn pop_accursed(&mut self) -> Option<Rc<RefCell<Cell>>> {
        let conns = Cell::as_connections(Box::new((*self.accursed).borrow_mut().clone()));
        let res = Rc::clone(&self.accursed);
        let curr = (*self.accursed).borrow_mut().clone();

        match curr.get_negward(self.d_cursor.clone()){
            None => return None,
            Some(i) => {
                self.accursed = Rc::clone(&i)
            }
        }

        for d in conns.0.keys() {
            let next = conns.0.get(&d);
            let prev = conns.1.get(&d);
            match prev {
                None => (),
                Some(v) => {
                    (*(next.unwrap())).borrow_mut().set_negward(d.clone(), Rc::clone(&v));
                    (*v).borrow_mut().set_posward(d.clone(), Rc::clone(&next.unwrap()));
                }
            }
        }

        Some(res)
    }

    pub fn shift_accursed(&mut self) -> Option<Rc<RefCell<Cell>>> {
        let conns = Cell::as_connections(Box::new((*self.accursed).borrow_mut().clone()));
        let res = Rc::clone(&self.accursed);
        let curr = (*self.accursed).borrow_mut().clone();

        match curr.get_posward(self.d_cursor.clone()){
            None => return None,
            Some(i) => {
                self.accursed = Rc::clone(&i)
            }
        }

        for d in conns.0.keys() {
            let next = conns.0.get(&d);
            let prev = conns.1.get(&d);
            match prev {
                None => (),
                Some(v) => {
                    (*(next.unwrap())).borrow_mut().set_negward(d.clone(), Rc::clone(&v));
                    (*v).borrow_mut().set_posward(d.clone(), Rc::clone(&next.unwrap()));
                }
            }
        }

        Some(res)
    }

}

impl Iterator for IterRank {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if self.end {
            return None;
        }

        let ac = (*self.accursed).borrow_mut().clone();
        // let res = Some(Cell::as_content(Box::new(ac)));
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
        Some(ac)
    }
}

impl DoubleEndedIterator for IterRank {

    fn next_back(&mut self) -> Option<Cell> {
        if self.end {
            return None;
        }

        let ac = (*self.accursed).borrow_mut().clone();
        // let res = Some(Cell::as_content(Box::new(ac)));
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
        Some(ac)
    }
}
