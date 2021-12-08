// wasm
use wasm_bindgen::prelude::*;

mod gen_maze;
mod path_finders;
mod util;

mod window;
use window::run;

#[wasm_bindgen]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    if let Err(_err) = run() {}
}

pub fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
    }
}
