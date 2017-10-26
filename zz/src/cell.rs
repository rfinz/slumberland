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

    pub fn set_posward(&mut self, dimension: Dimension, cell: Cell) {
        match self.posward {
            Some(ref mut i) => {
                i.insert(dimension, Rc::new(RefCell::new(cell)));
            },
            None => {
                let mut hm = HashMap::new();
                hm.insert(dimension, Rc::new(RefCell::new(cell)));
                self.posward = Some(hm);
            },
        };
    }

    pub fn set_negward(&mut self, dimension: Dimension, cell: Cell) {
        match self.negward {
            Some(ref mut i) => {
                i.insert(dimension, Rc::new(RefCell::new(cell)));
            },
            None => {
                let mut hm = HashMap::new();
                hm.insert(dimension, Rc::new(RefCell::new(cell)));
                self.negward = Some(hm);
            },
        };
    }

    pub fn get_posward(&mut self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
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

}
