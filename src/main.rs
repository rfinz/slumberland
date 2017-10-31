extern crate zz;
use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

fn main() {
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    let n = Cell::new(CellType::Monad("Python".to_string()));
    let n2 = Cell::new(CellType::Function("map".to_string()));
    let mut top = Topology::new(curse.clone(), ac);
    top.insert_posward(curse.clone(), n);
    top.insert_posward(curse.clone(), n2);
    for i in top.iter_rank(curse.clone()) {
        println!("{:?}", i);
    }
}
