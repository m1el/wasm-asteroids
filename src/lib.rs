pub mod time;
pub mod alloc;
pub mod eventloop;
mod math;
mod ship;

extern "C" {
    #[allow(dead_code)]
    fn alert(n: f64);
    fn puts(ptr: *const u8, len: usize);
    fn svg_set_path(ptr: *const u8, len: usize);
}

fn putstr(s: &str) {
    unsafe { puts(s.as_ptr(), s.len()) };
}
fn update_svg(s: &str) {
    unsafe { svg_set_path(s.as_ptr(), s.len()) };
}

use time::{Instant};
use eventloop::{Event, EventLoop};

use std::fmt::Write;
use math::{Vec2D};
use ship::{Inputs, Config, Ship};

struct Game {
    ship: Ship,
    inputs: Inputs,
    config: Config,
}

const SHIP_POINTS: [Vec2D; 6] = [
    Vec2D { x: 10.0, y: 0.0 },
    Vec2D { x: -10.0, y: -5.0 },
    Vec2D { x: -8.0, y: -2.5 },
    Vec2D { x: -8.0, y: 2.5 },
    Vec2D { x: -10.0, y: 5.0 },
    Vec2D { x: 10.0, y: 0.0 },
];
const FLARE: [Vec2D; 3] = [
    Vec2D { x: -8.0, y: 1.5 },
    Vec2D { x: -12.0, y: 0.0 },
    Vec2D { x: -8.0, y: -1.5 },
];

fn draw_ship(buf: &mut String, offset: Vec2D, inputs: &Inputs, ship: &Ship) {
    for (i, p) in SHIP_POINTS.iter().enumerate() {
        let c = if i == 0 { 'M' } else { 'L' };
        let p_c = p.scale(2.0).rotate(ship.angle) + offset + ship.pos;
        write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
            .expect("could not write string?");
    }
    if inputs.forward || inputs.backward {
        for (i, p) in FLARE.iter().enumerate() {
            let c = if i == 0 { 'M' } else { 'L' };
            let p_c = p.scale(2.0).rotate(ship.angle) + offset + ship.pos;
            write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
                .expect("could not write string?");
        }
    }
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
            field_size: Vec2D { x: 1280.0, y: 720.0 },
        }
    });
    game.ship.pos = game.config.field_size.scale(0.5);
    game.ship.angle = std::f64::consts::PI * -0.5;

    let _start = Instant::now();

    let mut event_loop = EventLoop::new(Box::new(move |event, event_loop| {
        let game = game.as_mut();
        match event {
            Event::Destroyed => {},
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
                let mut buf = String::new();
                let offset_x = if ship.pos.x * 2.0 < game.config.field_size.x { 1.0 } else { -1.0 } * game.config.field_size.x;
                let offset_y = if ship.pos.y * 2.0 < game.config.field_size.y { 1.0 } else { -1.0 } * game.config.field_size.y;
                let offsets = [
                    Vec2D { x: 0.0, y: 0.0 },
                    Vec2D { x: offset_x, y: 0.0 },
                    Vec2D { x: 0.0, y: offset_y },
                    Vec2D { x: offset_x, y: offset_y }
                ];
                for offset in offsets.iter() {
                    draw_ship(&mut buf, *offset, &game.inputs, &ship);
                }
                update_svg(&buf);
                event_loop.request_animation_frame();
            },
        }
    }));
    putstr("event loop started");
    event_loop.request_animation_frame();
}
