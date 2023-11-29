mod drone;
mod simulation;
mod utils;

use js_sys::{Array};
use rand::Rng;
use wasm_bindgen::prelude::*;
use simulation::Simulation;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn create_random_array(drone_cnt: u32) -> Array {
    let mut rng = rand::thread_rng();
    let values: Array = Array::new();

    for _ in 0..drone_cnt {
        let value = rng.gen::<f64>();
        values.push(&JsValue::from_f64(value));
    }

    values
}

fn get_drone_positions(sim: &Simulation) -> Array { 
    let values:Array = sim.drones.iter().map(|drone| {
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
    values
}

static mut SIMULATION: Option<Simulation> = None;

#[wasm_bindgen]
pub fn start_sim(drone_cnt: u32) -> Array {
    unsafe {
        SIMULATION = Some(Simulation::new(drone_cnt,0.01));
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

