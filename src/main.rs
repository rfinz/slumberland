extern crate zz;
extern crate memories;

use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Clone, Copy)]
struct Universe {
    topology: Signal<Topology>,
}

#[component]
fn App() -> Element {
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    let mut top: Signal<Topology> = use_signal(|| Topology::new(curse.clone(), ac));
    use_context_provider(|| Universe { topology:top });
    
    tracing::debug!("Rendering!");
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class:"flex flex-column",
            for c in top.read().iter_rank(curse.clone()){
                CellUI{ cell:c, dimension:curse.clone()}
            }
        }
    }
           
}

#[component]
fn CellUI(cell: Rc<RefCell<Cell>>, dimension: Dimension) -> Element {
    let borrowed = cell.borrow().clone();
    let uuid = borrowed.uuid;
    let ct = Box::new(borrowed).as_content();
    let negref = Rc::clone(&cell);
    let posref = Rc::clone(&cell);
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
            div { class:"w-100 bb pb2 mb2", "{content} {uuid}"}
            div {
                class: "flex justify-between",
                div {
                    class:"w-10 pointer",
                    onclick: move |evt| {
                        consume_context::<Universe>().topology.write().insert_negward_at(Dimension::new("cursor".to_string()), Rc::clone(&negref), Cell::new(CellType::Vertex));
                        consume_context::<Universe>().topology.write().reset_to_head(Dimension::new("cursor".to_string()));
                    },
                    "-"
                }
                div { class:"w-50", "cursor" }
                div {
                    class:"w-10 pointer",
                    onclick: move |evt| {
                        consume_context::<Universe>().topology.write().insert_posward_at(Dimension::new("cursor".to_string()), Rc::clone(&posref), Cell::new(CellType::Preload));
                        consume_context::<Universe>().topology.write().reset_to_head(Dimension::new("cursor".to_string()));
                    },
                    "+"
                }
            }
        }
    }
}
