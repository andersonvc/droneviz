use std::f64::consts::PI;

pub struct Drone {
    pub x_pos: f64,
    pub y_pos: f64,
    pub heading: f64,
    pub update_rate: f64,
    x_vel: f64,
    y_vel: f64,
    max_accel:f64,
    curr_accel:f64,
    curr_time:f64,
}

impl Drone {
    pub fn new(x_pos: f64, y_pos: f64, heading: f64, update_rate: f64) -> Self {
        Drone {
            x_pos,
            y_pos,
            heading: heading % (2.0 * PI),
            update_rate,
            x_vel: 0.0,
            y_vel: 0.0,
            max_accel: 0.01,
            curr_accel: 0.0,
            curr_time: 0.0,
        }
    }

    pub fn update_acceleration(&mut self, target:f64) {
        const ACCEL_STEP: f64 = 0.001;
        if self.curr_accel < target {
            self.curr_accel = (self.curr_accel + ACCEL_STEP).max(self.max_accel);
        } else {
            self.curr_accel = (self.curr_accel - ACCEL_STEP).min(0.0);
        }
    }

    pub fn turn_left(&mut self) {
        self.heading -= PI / 260.0;
        self.heading %= 2.0 * PI;
    }

    pub fn turn_right(&mut self) {
        self.heading += PI / 260.0;
        self.heading %= 2.0 * PI;
    }

    pub fn update_position(&mut self) {
        self.x_pos += self.x_vel*self.update_rate;//+0.5*self.curr_accel*self.heading.cos()*self.update_rate*self.update_rate;
        self.y_pos += self.y_vel*self.update_rate;//+0.5*self.curr_accel*self.heading.sin()*self.update_rate*self.update_rate;
        self.x_vel += self.curr_accel * self.heading.cos()*self.update_rate;
        self.y_vel += self.curr_accel * self.heading.sin()*self.update_rate;

        self.x_vel = self.x_vel.max(-0.15).min(0.15);
        self.y_vel = self.y_vel.max(-0.15).min(0.15);
    }

    pub fn return_to_center(&mut self, tracking_target: &Drone) {
        let target_center_x = &tracking_target.x_pos;
        let target_center_y = &tracking_target.y_pos;

        fn compute_angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
            let dx = x2 - x1;
            let dy = y2 - y1;
            dy.atan2(dx)
        }

        fn compute_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
            let dx = x2 - x1;
            let dy = y2 - y1;
            (dx.powi(2) + dy.powi(2)).sqrt()
        }

        let mut curr_angle = compute_angle(self.x_pos,self.y_pos,*target_center_x,*target_center_y)%(2.0*PI);
        if curr_angle<0.0 {
            curr_angle = (2.0*PI)+curr_angle;
        }

        let mut required_angle = curr_angle-self.heading;
        if required_angle<0.0{
            required_angle = (2.0*PI)+required_angle;
        }
        if (required_angle<PI) || (required_angle>2.0*PI)  {
            self.turn_right();
        } else {
            self.turn_left();
        }

        let curr_dist = compute_distance(self.x_pos, self.y_pos, *target_center_x, *target_center_y);
        let target_accel = curr_dist*self.max_accel*100.0;
        self.update_acceleration(target_accel);

    }

    pub fn preconfigured_motion(&mut self) {
        self.x_pos = 0.5 + (2.0*PI*(self.curr_time)/50.0).sin()*0.25;
        self.curr_time += self.update_rate;
        self.curr_time %= 100.0;
    }

}


