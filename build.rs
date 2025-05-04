use std::path::Path;
use std::fs;

use lightningcss::{
    stylesheet::{ ParserOptions, PrinterOptions },
    bundler::{ Bundler, FileProvider },
    css_modules::Config
};

fn main(){
    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions {
        css_modules:Some(Config::default()),
        ..ParserOptions::default()
    });
    let stylesheet = bundler.bundle(Path::new("assets/main_pre.css")).unwrap();
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    let dest_path = Path::new("assets/main.css");
    fs::write(&dest_path, res.code.as_bytes()).unwrap();
}
