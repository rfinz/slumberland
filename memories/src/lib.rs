extern crate zz;
extern crate itertools;
extern crate uuid;

use std::fs::File;
use std::io::Write;
use itertools::{Itertools, merge};
use uuid::Uuid;

use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

pub fn memorize(topology: &Topology, dimension: Dimension) {
    for i in topology.iter_rank(dimension) {
        let cell_box = Box::new(i);
        if let Err(_) = write_cell(cell_box) {

        }
    }
}

pub fn write_cell(cell_box: Box<Cell>) -> std::io::Result<()> {
    let name = cell_box.uuid.hyphenated().to_string();
    let directory = "data";
    let mut f = File::create(format!("{}/{}", &directory, &name))?;
    let content = Cell::as_content(cell_box.clone());
    let conns = Cell::as_connections(cell_box.clone());

    let content_string = match content {
        // format! is expensive compared to other string concatenation, but...
        CellType::Value(s) => format!("{:?}\n{:?}\n", "Value", s),
        CellType::Function(s) => format!("{:?}\n{:?}\n", "Function", s),
        CellType::Monad(s) => format!("{:?}\n{:?}\n", "Monad", s),
        CellType::Redirect => "Redirect\n".to_string(),
        CellType::Vertex => "Vertex\n".to_string()
    };

    f.write_all(content_string.as_bytes())?;

    for d in merge(conns.0.keys(), conns.1.keys()).unique() {
        let next = conns.0.get(&d);
        let prev = conns.1.get(&d);
        let &Dimension(ref d_value) = d;

        let mut res : String = "".to_owned();
        match prev {
            None => (),
            Some(v) => {
                res.push_str(&(*v.borrow().uuid.hyphenated().to_string()));
            }
        }
        res.push_str(&format!("|{}|", d_value));
        match next {
            None => (),
            Some(v) => {
                res.push_str(&(*v.borrow().uuid.hyphenated().to_string()));
            }
        }
        res.push_str(&"\n");

        f.write_all(res.as_bytes())?;
    }
    f.sync_all()?;
    Ok(())
}

// pub fn read_cell(cell_name: String) -> std::io::Result<Cell> {
//     let directory = "data";
//     let mut f = File::open(format!("{}/{}", &directory, &cell_name))?;




//     Ok(Cell::new(CellType::Vertex))
// }




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
