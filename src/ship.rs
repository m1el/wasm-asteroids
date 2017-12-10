use ::math::{Vec2D};
use ::game::{Inputs, Config};

#[derive(Debug)]
pub struct Ship {
    pub pos: Vec2D,
    pub speed: Vec2D,
    pub angle: f64,
    pub angular_speed: f64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            pos: Vec2D::zero(),
            speed: Vec2D::zero(),
            angle: 0.0,
            angular_speed: 0.0,
        }
    }
    pub fn tick(&mut self, inputs: &Inputs, config: &Config) {
        // drag
        let drag = self.speed.dot(&self.speed) * config.drag;
        self.speed -= self.speed.scale(drag * config.delta_t);

        let angular_drag = self.angular_speed * config.angular_drag;
        self.angular_speed -= angular_drag * config.delta_t;

        // inputs
        let accel_dir = match (inputs.forward, inputs.backward) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        if accel_dir != 0.0 {
            let accel = accel_dir * config.acceleration * config.delta_t;
            let accel = Vec2D { x: accel, y: 0.0 }.rotate(self.angle);
            self.speed += accel;
        }

        let rotate_dir = match (inputs.left, inputs.right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        if rotate_dir != 0.0 {
            let accel = rotate_dir * config.angular_accel * config.delta_t;
            self.angular_speed += accel;
        }

        // limiters
        let speed = self.speed.len();
        if speed > config.speed_limit {
            self.speed = self.speed.scale(1.0 * config.speed_limit / speed);
        }
        self.angular_speed = self.angular_speed.min(config.angular_limit).max(-config.angular_limit);

        // constrain position
        self.pos.clip(&config.field_size);

        // integration step
        self.pos += self.speed.scale(config.delta_t);
        self.angle += self.angular_speed * config.delta_t;
    }
}
