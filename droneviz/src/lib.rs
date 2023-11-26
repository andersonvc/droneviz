use rand::Rng;
use std::thread;
use std::time::Duration;

use js_sys::Array;

mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

pub fn create_random_array() -> Array {
    let mut rng = rand::thread_rng();
    let mut values: Array = Array::new();

    for _ in 0..50 {
        let value = rng.gen::<f64>();
        values.push(&JsValue::from_f64(value));
    }

    values
}

#[wasm_bindgen]
pub fn start_sim() -> Array{
    create_random_array()
}

