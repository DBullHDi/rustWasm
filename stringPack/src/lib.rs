#![feature(use_extern_macros)]

extern crate wasm_bindgen;

//we connect to the new library
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="../functions")]
extern {
    fn appendString(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    appendString("Hello String");
}
