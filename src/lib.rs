pub mod time;
pub mod alloc;
pub mod eventloop;
mod math;
mod ship;

extern "C" {
    #[allow(dead_code)]
    fn alert(n: f64);
    fn puts(ptr: *const u8, len: usize);
    fn ship_set_position(x: f64, y: f64, a: f64);
}

fn putstr(s: &str) {
    unsafe { puts(s.as_ptr(), s.len()) };
}

use time::{Instant};
use eventloop::{Event, EventLoop};

use math::{Vec2D};
use ship::{Inputs, Config, Ship};

struct Game {
    ship: Ship,
    inputs: Inputs,
    config: Config,
}

#[no_mangle]
pub extern "C"
fn my_main() {
    let mut game = Box::new(Game {
        ship: Ship::new(),
        inputs: Inputs::new(),
        config: Config {
            acceleration: 250.0,
            speed_limit: 1000.0,
            drag: 0.000005,

            angular_accel: 4.0,
            angular_limit: 4.0,
            angular_drag: 0.05,

            delta_t: 1.0 / 60.0,
            field_size: Vec2D { x: 1000.0, y: 1000.0 },
        }
    });

    let _start = Instant::now();

    let mut event_loop = EventLoop::new(Box::new(move |event, event_loop| {
        let game = game.as_mut();
        match event {
            Event::MouseMove { x: _, y: _ } => {
                //putstr(&format!("x: {}, y: {}", x, y));
            },
            Event::KeyDown { code, chr: _, flags: _ } => {
                let inputs = &mut game.inputs;
                match code {
                    38 => inputs.forward = true,
                    40 => inputs.backward = true,
                    37 => inputs.left = true,
                    39 => inputs.right = true,
                    _ => {},
                }
            },
            Event::KeyUp { code, chr: _, flags: _ } => {
                let inputs = &mut game.inputs;
                match code {
                    38 => inputs.forward = false,
                    40 => inputs.backward = false,
                    37 => inputs.left = false,
                    39 => inputs.right = false,
                    _ => {},
                }
            },
            Event::AnimationFrame => {
                let ship = &mut game.ship;
                ship.tick(&game.inputs, &game.config);
                unsafe { ship_set_position(ship.pos.x, ship.pos.y, ship.angle) };
                event_loop.request_animation_frame();
            },
        }
    }));
    putstr("event loop started");
    event_loop.request_animation_frame();
}
