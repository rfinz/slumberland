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
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    let n = Cell::new(CellType::Monad("Python".to_string()));
    let n2 = Cell::new(CellType::Function("map".to_string()));
    let mut top = Topology::new(curse.clone(), ac);
    top.insert_posward(curse.clone(), n);
    top.accurse_posward();
    top.insert_posward(curse.clone(), n2);
    top.accurse_negward();
    println!("-----> BEFORE POP");
    memories::memorize(&top, curse.clone());
    top.accurse_posward();
    top.pop_accursed();
    println!("-----> AFTER POP");
    memories::memorize(&top, curse.clone());
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "title",
              h1 { "HotDog!" }
        }
        div { id: "buttons",
              button { id: "skip", "skip" }
              button { id: "save", "save" }
        }
    }
           
}
