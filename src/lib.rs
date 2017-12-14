pub mod time;
pub mod alloc;
pub mod eventloop;
//mod rand;
mod geom;
mod math;
mod ship;
mod input;
mod game;
mod render_path;

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

use game::{Game};
use render_path::{render_game};

#[no_mangle]
pub extern "C"
fn my_main() {
    let mut game = Box::new(Game::new());

    let _start = Instant::now();

    let mut event_loop = EventLoop::new(Box::new(move |event, event_loop| {
        let game = game.as_mut();
        match event {
            Event::Destroyed => {},
            Event::MouseMove { x: _, y: _ } => {
                //putstr(&format!("x: {}, y: {}", x, y));
            },
            Event::KeyDown { code, chr: _, flags: _ } => {
                game.inputs.key_down(code, &game.config);
            },
            Event::KeyUp { code, chr: _, flags: _ } => {
                game.inputs.key_up(code, &game.config);
            },
            Event::AnimationFrame => {
                game.tick();
                let mut buf = String::new();
                render_game(&mut buf, game);
                update_svg(&buf);
                event_loop.request_animation_frame();
            },
        }
    }));
    putstr("event loop started");
    event_loop.request_animation_frame();
}
