extern crate zz;
extern crate memories;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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
    pub topology: Signal<Topology>
}

#[component]
fn App() -> Element {
    let curse = Dimension::new("cursor".to_string());
    let ac = Cell::new(CellType::Vertex);
    let mut top = use_signal(|| Topology::new(curse.clone(), ac));
    use_context_provider(|| Universe { topology:top });

    tracing::debug!("***---Rendering!---***");
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class:"flex flex-column",
            div {
                class:"flex flex-column",
                for (i,c) in use_context::<Universe>().topology.read().iter_rank(curse.clone()).enumerate(){
                    div {
                        class:"flex",
                        CellUI{ cell:Rc::clone(&c) }
                    }
                }
            }
            div {
                class:"flex",
                div {
                    class:"w-100 pointer tc pt2 mt2 bt",
                    onclick: move |evt| {
                        use_context::<Universe>().topology.write().insert_negward(
                            curse.clone(),
                            Cell::new(CellType::Preload)
                        );
                    },
                    "*"
                }
            }
        }
    }      
}

#[component]
fn CellUI(cell: Rc<RefCell<Cell>>) -> Element {
    let borrowed = cell.borrow().clone();
    let uuid = borrowed.uuid;
    let ct = Box::new(borrowed).as_content();
    let mut top =  consume_context::<Universe>().topology;
    let content = match ct {
        CellType::Value(v) => v,
        CellType::Function(f) => f, 
        CellType::Monad(m) => m, 
        CellType::Redirect => "Redirect".to_string(), 
        CellType::Vertex => "Vertex".to_string(), 
        CellType::Preload => "Preload".to_string() 
    };
    tracing::debug!("-> vvv Cell: {:?}-{:?}", uuid.to_string(), content);
    rsx! {
        div {
            class: "w-25 cell tc",
            div { class:"w-100 bb pb2 mb2", "{content}"}
            div { class:"w-100 bb pb2 mb2", "{uuid}"}
            div {
                class: "flex justify-between",
                div {
                    class:"w-10 pointer",
                    onclick: move |evt| {
                        top.write().insert_negward(
                            Dimension::new("cursor".to_string()),
                            Cell::new(CellType::Preload)
                        )
                    },
                    "-"
                }
                div { class:"w-50", "cursor" }
                div {
                    class:"w-10 pointer",
                    onclick: move |evt| {
                        top.write().insert_posward(
                            Dimension::new("cursor".to_string()),
                            Cell::new(CellType::Preload)
                        )
                    },
                    "+"
                }
            }
        }
        {
            let pos = match (*cell).borrow().get_posward(Dimension::new("cursor".to_string())) {
                None => "None".to_string(),
                Some(k) => k.borrow().uuid.to_string()
            };
            let neg = match (*cell).borrow().get_negward(Dimension::new("cursor".to_string())) {
                None => "None".to_string(),
                Some(k) => k.borrow().uuid.to_string()
            };
            tracing::debug!("-> ^^^ neg: {:?} pos: {:?}", neg, pos);
        }  
        
    }
    
}
