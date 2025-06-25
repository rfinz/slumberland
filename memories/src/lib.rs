extern crate zz;
extern crate itertools;
extern crate uuid;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use itertools::{Itertools, merge};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;

use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

pub fn memorize(topology: &Topology, dimension: Dimension) {
    for i in topology.iter_rank(dimension) {
        let cell = i.borrow().clone();
        let cell_box = Box::new(cell);
        let cell_name = Box::clone(&cell_box).uuid.hyphenated().to_string();
        if let Err(_) = write_cell(cell_box) {

        }
        if let Err(_) = read_cell(cell_name) {

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
        CellType::Value(s) => format!("{}\n{}\n", "Value", s.to_string()),
        CellType::Function(s) => format!("{}\n{}\n", "Function", s.to_string()),
        CellType::Monad(s) => format!("{}\n{}\n", "Monad", s.to_string()),
        CellType::Redirect => "Redirect\n\n".to_string(),
        CellType::Vertex => "Vertex\n\n".to_string(),
        CellType::Preload => "Preload\n\n".to_string()
    };

    f.write_all(content_string.as_bytes())?;

    for d in merge(conns.0.keys(), conns.1.keys()).unique() {
        let next = conns.0.get(&d);
        let prev = conns.1.get(&d);
        let Dimension{name: d_value} = d;

        let mut res : String = "".to_owned();
        match prev {
            None => (),
            Some(v) => {
                res.push_str(&(*v.borrow().uuid.hyphenated().to_string()));
            }
        }
        res.push_str(&format!("->{}->", d_value));
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

pub fn read_cell(cell_name: String) -> std::io::Result<Cell> {
    let directory = "data";
    let mut f = File::open(format!("{}/{}", &directory, &cell_name))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    /*
    Instead of an arbitrary (and limiting) record delimiter I should
    write the record length into a fixed number of bytes at the beginning
    of a field and then read that instead. Gimme a sec, ya know?
     */

    let mut splits = contents.split("\n").collect::<Vec<&str>>();
    let cell_uuid =  match Uuid::parse_str(cell_name.as_str()) {
            Err(_e) => Uuid::new_v4(), // not sure about this choice
            Ok(u) => u
    };
    let cell_type_name = splits.remove(0);
    let cell_content = splits.remove(0);
    let mut posward = HashMap::new();
    let mut negward = HashMap::new();

    let cell_type = match cell_type_name {
        "Value" => CellType::Value(cell_content.to_string()),
        "Function" => CellType::Function(cell_content.to_string()),
        "Monad" => CellType::Monad(cell_content.to_string()),
        "Redirect" => CellType::Redirect,
        "Vertex" => CellType::Vertex,
        "Preload" => CellType::Preload,
        &_ => CellType::Vertex
    };

    for s in splits.iter() {
        let conns = s.split("->").collect::<Vec<&str>>();
        if conns.len() <= 1 {continue;};
        match Uuid::parse_str(conns[0]) {
            Err(_e) => None,
            Ok(u) => negward.insert(Dimension::new(conns[1].to_string()), Rc::new(RefCell::new(Cell::from_uuid(u))))
        };
        match Uuid::parse_str(conns[2]) {
            Err(_e) => None,
            Ok(u) => posward.insert(Dimension::new(conns[1].to_string()), Rc::new(RefCell::new(Cell::from_uuid(u))))
        };
    }

    println!("{:?}", cell_uuid.hyphenated().to_string());
    Ok(Cell::from_parts(posward, negward, cell_uuid, cell_type))
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
