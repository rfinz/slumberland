use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use dimension::Dimension;

#[derive(Clone, Debug)]
pub enum CellType {
    Value(String), // Text, Numeral, File, Image, etc.
    Function(String), // Transforms previous cell with code
    Monad(String), // Performs some cell-independent function ("side-effects")
    Vertex // Holds a place in a topology without representing data
}

#[derive(Clone, Debug)]
pub struct Cell {
    posward: Option<HashMap<Dimension, Rc<RefCell<Cell>>>>,
    negward: Option<HashMap<Dimension, Rc<RefCell<Cell>>>>,
    content: CellType
}

impl Cell {
    pub fn new(content: CellType) -> Self {
        Cell {
            posward: None,
            negward: None,
            content
        }
    }

    pub fn as_content(self: Box<Self>) -> CellType {
        self.content
    }

    pub fn set_posward(&mut self, dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        match self.posward {
            Some(ref mut i) => {
                i.insert(dimension, cell);
            },
            None => {
                let mut hm = HashMap::new();
                hm.insert(dimension, cell);
                self.posward = Some(hm);
            },
        };
    }

    pub fn set_negward(&mut self, dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        match self.negward {
            Some(ref mut i) => {
                i.insert(dimension, cell);
            },
            None => {
                let mut hm = HashMap::new();
                hm.insert(dimension, cell);
                self.negward = Some(hm);
            },
        };
    }

    pub fn get_posward(&self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        let op = match self.posward {
            Some(ref i) => match i.get(&dimension) {
                None => None,
                Some(k) => Some(Rc::clone(k)),
            },
            None => None,
        };
        op
    }

    pub fn get_negward(&self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        let op = match self.negward {
            Some(ref i) => match i.get(&dimension) {
                None => None,
                Some(k) => Some(Rc::clone(k)),
            },
            None => None,
        };
        op
    }

    pub fn as_connections(self: Box<Self>) -> (
        Option<HashMap<Dimension, Rc<RefCell<Cell>>>>,
        Option<HashMap<Dimension, Rc<RefCell<Cell>>>>) {

        let res_pos = self.clone();
        let res_neg = self.clone();

        (res_pos.posward, res_neg.negward)

    }

    pub fn unlink_posward(&mut self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        let op = match self.posward {
            Some(ref mut i) => match i.remove(&dimension) {
                None => None,
                Some(k) => Some(Rc::clone(&k)),
            },
            None => None,
        };
        op
    }

    pub fn unlink_negward(&mut self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        let op = match self.negward {
            Some(ref mut i) => match i.remove(&dimension) {
                None => None,
                Some(k) => Some(Rc::clone(&k)),
            },
            None => None,
        };
        op
    }

}
