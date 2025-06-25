use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use crate::cell::{Cell, CellType};
use crate::dimension::Dimension;

use uuid::Uuid;

pub struct Topology {
    d_cursor: Dimension,
    accursed: Rc<RefCell<Cell>>,
    dimensions: Vec<Dimension>,
    pub fabric: HashMap<Uuid, Rc<RefCell<Cell>>>
}

pub struct IterRank {
    rank: Dimension,
    accursed: Rc<RefCell<Cell>>,
    end: bool
}

impl Topology {
    pub fn new(d_cursor: Dimension, cell: Cell) -> Self {
        let ac = Rc::new(RefCell::new(cell));
        Topology {
            d_cursor: d_cursor.clone(),
            accursed: Rc::clone(&ac),
            dimensions: vec![d_cursor],
            fabric: HashMap::from([(ac.borrow().uuid, Rc::clone(&ac))])
        }
    }

    pub fn add_cell(&mut self, cell_type:CellType) -> Uuid {
        let cell = Cell::new(cell_type);
        let reference = Rc::new(RefCell::new(cell));
        let key = (*reference).borrow().uuid.clone();
        self.fabric.insert(key, reference);
        key
    }
    
    // if data stays in the topology, return a pointer
    pub fn get_head(&self, rank: Dimension) -> Rc<RefCell<Cell>> {
        match self.iter_rank(rank).rev().last() {
            None => Rc::clone(&self.accursed),
            Some(i) => i
        }
    }

    pub fn get_tail(&self, rank: Dimension) -> Rc<RefCell<Cell>> {
        match self.iter_rank(rank).last() {
            None => Rc::clone(&self.accursed),
            Some(i) => i
        }
    }
    
    pub fn get_accursed(&self) -> Rc<RefCell<Cell>>{
        Rc::clone(&self.accursed)
    }

    pub fn reset_to_head(&mut self, rank:Dimension) {
        self.accursed = self.get_head(rank);   
    }

    pub fn reset_to_tail(&mut self, rank:Dimension) {
        self.accursed = self.get_tail(rank);   
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
        {
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
        self.accursed = Rc::clone(&cell);
    }

    pub fn insert_negward(&mut self, dimension: Dimension, mut cell: Cell) {
        cell.set_posward(dimension.clone(), Rc::clone(&self.accursed));
        let cell = Rc::new(RefCell::new(cell));
        {
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
        self.accursed = Rc::clone(&cell);
    }

    pub fn insert_posward_at(&mut self, dimension: Dimension, at: Uuid, cell: Uuid) {
        let rcat = self.fabric.get(&at).unwrap(); // todo create from UUID if missing
        let rcc = self.fabric.get(&cell).unwrap(); // todo create from UUID if missing
        let mut borrowed_c = (*rcc).borrow_mut();
        let mut borrowed_at = (*rcat).borrow_mut();
        
        borrowed_c.set_negward(dimension.clone(), Rc::clone(&rcat));
        
        match borrowed_at.get_posward(dimension.clone()){
            None => {
               borrowed_at.set_posward(dimension.clone(), Rc::clone(&rcc));
            },
            Some(i) => {
                borrowed_c.set_posward(dimension.clone(), Rc::clone(&i));
                (*i).borrow_mut().set_negward(dimension.clone(), Rc::clone(&rcc));
                borrowed_at.set_posward(dimension.clone(), Rc::clone(&rcc));
            }
        };
        self.accursed = Rc::clone(&rcat);
    }

    pub fn insert_negward_at(&mut self, dimension: Dimension, at: Uuid, cell: Uuid) {
        let rcat = self.fabric.get(&at).unwrap(); // todo create from UUID if missing
        let rcc = self.fabric.get(&cell).unwrap(); // todo create from UUID if missing
        let mut borrowed_c = (*rcc).borrow_mut();
        let mut borrowed_at = (*rcat).borrow_mut();
        
        //let c = Rc::new(RefCell::new(cell.clone()));
        //let mut borrowed_at = (*at).borrow_mut();
        //let mut borrowed_c = (*c).borrow_mut();
        //self.fabric.insert(borrowed_c.uuid, Rc::clone(&c));
        
        borrowed_c.set_posward(dimension.clone(), Rc::clone(&rcat));
        
        match borrowed_at.get_negward(dimension.clone()){
            None => {
                borrowed_at.set_negward(dimension.clone(), Rc::clone(&rcc));
            },
            Some(i) => {
                borrowed_c.set_negward(dimension.clone(), Rc::clone(&i));
                (*i).borrow_mut().set_posward(dimension.clone(), Rc::clone(&rcc));
                borrowed_at.set_negward(dimension.clone(), Rc::clone(&rcc));
            }
        };
        self.accursed = Rc::clone(&rcat);
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

/// IterRank processes from the currently accursed cell, and ends when it returns None.

impl Iterator for IterRank {
    type Item = Rc<RefCell<Cell>>;

    fn next(&mut self) -> Option<Rc<RefCell<Cell>>> {
        if self.end {
            return None;
        }

        let ac = Rc::clone(&self.accursed);
        // let res = Some(Cell::as_content(Box::new(ac)));
        let n = match (*self.accursed).borrow().get_posward(self.rank.clone()) {
            None => {
                self.end = true;
                Rc::clone(&self.accursed)
            },
            Some(i) => {
                Rc::clone(&i)
            },
        };
        self.accursed = n;
        Some(ac)
    }
}

impl DoubleEndedIterator for IterRank {

    fn next_back(&mut self) -> Option<Rc<RefCell<Cell>>> {
        if self.end {
            return None;
        }

        let ac = Rc::clone(&self.accursed);
        // let res = Some(Cell::as_content(Box::new(ac)));
        let n = match (*self.accursed).borrow().get_negward(self.rank.clone()) {
            None => {
                self.end = true;
                Rc::clone(&self.accursed)
            },
            Some(i) => {
                Rc::clone(&i)
            },
        };
        self.accursed = n;
        Some(ac)
    }
}
