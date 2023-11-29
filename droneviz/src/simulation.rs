use rand::Rng;
use std::f64::consts::PI;
use crate::drone::Drone;





pub struct Simulation {
    pub drones: Vec<Drone>,
    pub target: Drone,
    pub update_rate: f64,
}

impl Simulation {
    pub fn new(drone_cnt: u32, update_rate: f64) -> Self {
        let mut tmp_rng = rand::thread_rng();
        let tmp_target = Drone::new(0.5, 0.5, 0.0, update_rate);
        let mut tmp_drones = Vec::new();
        for _ in 0..drone_cnt {
            let x_pos = tmp_rng.gen::<f64>();
            let y_pos = tmp_rng.gen::<f64>();
            let heading = tmp_rng.gen::<f64>() * PI;
            tmp_drones.push(Drone::new(x_pos, y_pos, heading,update_rate));
        }
        
        Simulation {
            drones: tmp_drones,
            target: tmp_target,
            update_rate: update_rate, // 10ms
        }
    }

    pub fn step(&mut self) {
        self.target.preconfigured_motion();
        for drone in self.drones.iter_mut() {

            drone.return_to_center(&self.target);
            drone.update_position();
            
        }
    }
}
