use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::dimension::Dimension;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Value(String), // Text, Numeral, File, Image, etc.
    Function(String), // Transforms previous cell with code
    Monad(String), // Performs some cell-independent function ("side-effects")
    Redirect, // Directs movement through the topology based on conditions
    Vertex, // Holds a place in a topology without representing data
    Preload // The contents and connections still need loading
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    posward: HashMap<Dimension, Rc<RefCell<Cell>>>,
    negward: HashMap<Dimension, Rc<RefCell<Cell>>>,
    pub uuid: Uuid,
    content: CellType
}

impl Cell {
    pub fn new(content: CellType) -> Self {
        Cell {
            posward: HashMap::new(),
            negward: HashMap::new(),
            uuid: Uuid::new_v4(),
            content: content
        }
    }

    pub fn from_parts(
        posward: HashMap<Dimension, Rc<RefCell<Cell>>>,
        negward: HashMap<Dimension, Rc<RefCell<Cell>>>,
        uuid: Uuid,
        content: CellType
    ) -> Self {
        Cell {
            posward: posward,
            negward: negward,
            uuid: uuid,
            content: content
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Cell {
            posward: HashMap::new(),
            negward: HashMap::new(),
            uuid: uuid,
            content: CellType::Preload
        }
    }

    pub fn as_content(self: Box<Self>) -> CellType {
        self.content
    }

    pub fn set_posward(&mut self, dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        self.posward.insert(dimension, cell);
    }

    pub fn set_negward(&mut self, dimension: Dimension, cell: Rc<RefCell<Cell>>) {
        self.negward.insert(dimension, cell);
    }

    pub fn get_posward(&self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        match self.posward.get(&dimension) {
            None => None,
            Some(k) => Some(Rc::clone(&k)),
        }
    }

    pub fn get_negward(&self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        match self.negward.get(&dimension) {
            None => None,
            Some(k) => Some(Rc::clone(&k)),
        }
    }

    pub fn as_connections(self: Box<Self>) -> (
        HashMap<Dimension, Rc<RefCell<Cell>>>,
        HashMap<Dimension, Rc<RefCell<Cell>>>) {

        let res_pos = self.clone();
        let res_neg = self.clone();

        (res_pos.posward, res_neg.negward)

    }

    pub fn unlink_posward(&mut self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        match self.posward.remove(&dimension) {
            None => None,
            Some(k) => Some(Rc::clone(&k)),
        }
    }

    pub fn unlink_negward(&mut self, dimension: Dimension) -> Option<Rc<RefCell<Cell>>> {
        match self.negward.remove(&dimension) {
            None => None,
            Some(k) => Some(Rc::clone(&k)),
        }
    }

}
