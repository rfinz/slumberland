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

#[derive(Clone, Copy)]
struct Universe {
    topology: Signal<Topology>,
}

#[component]
fn App() -> Element {
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    //let n = Cell::new(CellType::Monad("Python".to_string()));
    //let n2 = Cell::new(CellType::Function("map".to_string()));
    let mut top: Signal<Topology> = use_signal(|| Topology::new(curse.clone(), ac));
    //top.write().insert_posward(curse.clone(), n);
    //top.write().accurse_posward();
    //top.write().insert_posward(curse.clone(), n2);
    //top.write().accurse_negward();
    use_context_provider(|| Universe { topology:top });
    //top.accurse_negward();
    
    
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
fn CellUI(cell: Cell, dimension: Dimension) -> Element {
    let uuid = cell.uuid;
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
            div { class:"w-100 bb pb2 mb2", "{content} {uuid}"}
            div {
                class: "flex justify-between",
                div {
                    class:"w-10",
                    onclick: move |evt| {
                        consume_context::<Universe>().topology.write().insert_negward(Dimension::new("cursor".to_string()), Cell::new(CellType::Vertex));
                        consume_context::<Universe>().topology.write().reset_to_head(Dimension::new("cursor".to_string()))
                    },
                    "-"
                }
                div { class:"w-50", "cursor" }
                div {
                    class:"w-10",
                    onclick: move |evt| consume_context::<Universe>().topology.write().insert_posward(Dimension::new("cursor".to_string()), Cell::new(CellType::Preload)),
                    "+"
                }
            }
        }
    }
}
