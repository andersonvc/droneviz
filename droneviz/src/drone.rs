struct Drone {
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
}

impl Drone {
    fn new(x_pos: f64, y_pos: f64) -> Self {
        Drone {
            x_pos,
            y_pos,
            x_vel: 0.0,
            y_vel: 0.0,
        }
    }

    fn get_pos(&self) -> (f64, f64) {
        (self.x_pos, self.y_pos)
    }

    fn update_position(&mut self) {
        let mut rng = rand::thread_rng();
        self.x_pos += rng.gen_range(-1.0..1.0);
        self.y_pos += rng.gen_range(-1.0..1.0);
    }
}


fn simulation() {
    let mut drones: Vec<Drone> = Vec::new();

    for _ in 0..50 {
        let x_pos = rand::thread_rng().gen_range(0.0..100.0);
        let y_pos = rand::thread_rng().gen_range(0.0..100.0);
        drones.push(Drone::new(x_pos, y_pos));
    }

    loop {
        for drone in &mut drones {
            drone.update_position();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
