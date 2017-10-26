extern crate zz;
use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

fn main() {
    let curse = Dimension::new("cursor".to_string());
    let fake = Dimension::new("fake".to_string());
    let mut ac = Cell::new(CellType::Vertex);
    let mut n = Cell::new(CellType::Monad("Python".to_string()));
    let n2 = Cell::new(CellType::Function("map".to_string()));
    n.set_posward(fake, n2);
    ac.set_posward(curse.clone(), n);
    let top = Topology::new(curse.clone(), ac);
    for i in top.iter_rank(curse.clone()) {
        println!("{:?}", i);
    }
}
