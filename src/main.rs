extern crate zz;
extern crate memories;

use zz::topology::Topology;
use zz::dimension::Dimension;
use zz::cell::{Cell, CellType};

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
static CSS: Asset = asset!("/assets/bundled.css");
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
    
    
    tracing::debug!("Rendering!");
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class:"flex flex-column",
            for c in top.iter_rank(curse.clone()){
                CellUI{ cell:c }
            }
        }
    }
           
}

#[component]
fn CellUI(cell: Cell) -> Element {
    let ct = Box::new(cell).as_content();
    let content = match ct {
        CellType::Value(v) => v, 
        CellType::Function(f) => f, 
        CellType::Monad(m) => m, 
        CellType::Redirect => "Redirect".to_string(), 
        CellType::Vertex => "Vertex".to_string(), 
        CellType::Preload => "Preload".to_string() 
    };
    rsx! {
        div {
            class: "w-25 cell tc",
            div { class:"w-100 bb pb2 mb2", "{content}"}
            div {
                class: "flex justify-between",
                div { class:"w-10", "-" }
                div { class:"w-50", "cursor" }
                div { class:"w-10", "+" }
            }
        }
    }
}
