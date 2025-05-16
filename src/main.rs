extern crate zz;
extern crate memories;

use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
static CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    let n = Cell::new(CellType::Monad("Python".to_string()));
    let n2 = Cell::new(CellType::Function("map".to_string()));
    let mut top = Topology::new(curse.clone(), ac);
    top.insert_posward(curse.clone(), n);
    top.accurse_posward();
    top.insert_posward(curse.clone(), n2);
    top.accurse_negward();
    top.accurse_negward();
    //println!("-----> BEFORE POP");
    //memories::memorize(&top, curse.clone());
    //top.accurse_posward();
    //top.pop_accursed();
    //println!("-----> AFTER POP");
    memories::memorize(&top, curse.clone());
    let contents = top.iter_rank(curse.clone()).map(|i| {
        let ct = Box::new(i).as_content();
        match ct {
            CellType::Value(v) => v, 
            CellType::Function(f) => f, 
            CellType::Monad(m) => m, 
            CellType::Redirect => "Redirect".to_string(), 
            CellType::Vertex => "Vertex".to_string(), 
            CellType::Preload => "Preload".to_string() 
        }
    });
    
    tracing::debug!("Rendering!");
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            for c in contents {
                div { class: "cell", "{c}"}
            }
        }
    }
           
}
