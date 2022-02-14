mod grapher;
mod math;
mod parser;
mod render;
mod shaders;
mod utils;

#[cfg(test)]
mod tests;

use grapher::Grapher;
use js_sys::Array;
use utils::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlCanvasElement;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    #[wasm_bindgen]
    pub fn theme(id: u32) -> Array;
}

#[wasm_bindgen]
pub fn canvas_init(id: &str) -> Grapher {
    set_panic_hook();

    log(format!("Getting {}", id).as_str());

    {
        let a: i32 = 420;
        let ptr: *const i32 = 420 as *const i32;

        unsafe {
            let b = *ptr;
            println!("{}", b);
        }
    }
    // gets HTML canvas reference
    let document = window().document().unwrap_throw();

    let canvas = document
        .query_selector(id)
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap_throw();

    Grapher::new(canvas)
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
