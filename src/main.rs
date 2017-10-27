extern crate zz;
use std::cell::RefCell;
use std::rc::Rc;
use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

fn main() {
    let curse = Dimension::new("cursor".to_string());
    let mut ac = Cell::new(CellType::Vertex);
    let mut n = Cell::new(CellType::Monad("Python".to_string()));
    let n2 = Cell::new(CellType::Function("map".to_string()));
    n.set_posward(curse.clone(), Rc::new(RefCell::new(n2)));
    ac.set_posward(curse.clone(), Rc::new(RefCell::new(n)));
    let top = Topology::new(curse.clone(), ac);
    for i in top.iter_rank(curse.clone()) {
        println!("{:?}", i);
    }
}
