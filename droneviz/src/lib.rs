use rand::Rng;
use js_sys::{Array}; // Declare JsValue

mod utils;
mod drone;//::Simulation;

//use crate::drone::Simulation; // Declare Simulation

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

pub fn create_random_array(drone_cnt: u32) -> Array {
    let mut rng = rand::thread_rng();
    let mut values: Array = Array::new();

    for _ in 0..drone_cnt {
        let value = rng.gen::<f64>();
        values.push(&JsValue::from_f64(value));
    }

    values
}

fn get_drone_positions(sim: &drone::Simulation) -> Array { 
    let mut values:Array = sim.drones.iter().map(|drone| {
        Array::of3(
            &JsValue::from_f64(drone.x_pos),
            &JsValue::from_f64(drone.y_pos),
            &JsValue::from_f64(drone.heading),
        )
    }).collect();

    let target:Array = Array::of3(
        &JsValue::from_f64(sim.target.x_pos),
        &JsValue::from_f64(sim.target.y_pos),
        &JsValue::from_f64(sim.target.heading),
    );

    values.push(&target);
    //    &JsValue::from_f64(sim.target.y_pos),
    //    &JsValue::from_f64(sim.target.heading));
    values




}

static mut SIMULATION: Option<drone::Simulation> = None;

#[wasm_bindgen]
pub fn start_sim(drone_cnt: u32) -> Array {
    unsafe {
        SIMULATION = Some(drone::Simulation::new(drone_cnt,0.01));
        let sim = SIMULATION.as_ref().unwrap();
        get_drone_positions(sim)
    }
}

#[wasm_bindgen]
pub fn step_sim() -> Array {
    unsafe {
        let sim = SIMULATION.as_mut().unwrap();
        sim.step();
        get_drone_positions(sim)
    }
}

