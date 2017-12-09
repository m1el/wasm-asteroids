use std::cell::{RefCell};
use std::collections::{HashMap};

extern "C" {
    fn event_loop_new() -> u32;
    fn event_loop_raf(id: u32);
    fn event_loop_shutdown(id: u32) -> bool;
}

const EVENT_ANIMATION_FRAME: u32 = 1;
const EVENT_MOUSE_MOVE: u32 = 2;
const EVENT_KEY_DOWN: u32 = 3;
const EVENT_KEY_UP: u32 = 4;

#[derive(Copy,Clone)]
pub enum Event {
    AnimationFrame,
    MouseMove { x: u32, y: u32 },
    KeyDown { code: u32, chr: Option<char>, flags: u32 },
    KeyUp { code: u32, chr: Option<char>, flags: u32 },
}

#[no_mangle]
pub extern "C"
fn event_loop_cb(id: u32, msg: u32, p0: u32, p1: u32, p2: u32) {
    EVENTLOOPS.with(|el| {
        let mut el = el.borrow_mut();
        let cb =
            if let Some(cb) = el.get_mut(&id) { cb }
            else { return; }; // Ignore unknown ids

        let event = match msg {
            EVENT_ANIMATION_FRAME => Event::AnimationFrame,
            EVENT_MOUSE_MOVE => Event::MouseMove { x: p0, y: p1 },
            EVENT_KEY_DOWN => Event::KeyDown { code: p0, chr: ::std::char::from_u32(p1), flags: p2 },
            EVENT_KEY_UP => Event::KeyUp { code: p0, chr: ::std::char::from_u32(p1), flags: p2 },
            _ => return,
        };
        let mut fake_event_loop = EventLoop { id: id };
        cb(event, &mut fake_event_loop);
    });
}

thread_local! {
    static EVENTLOOPS: RefCell<HashMap<u32, EventLoopCb>> = RefCell::new(HashMap::new());
}

pub type EventLoopCb = Box<FnMut(Event, &mut EventLoop)>;

pub struct EventLoop {
    id: u32,
}

impl EventLoop {
    pub fn new(cb: EventLoopCb) -> EventLoop {
        let id = unsafe { event_loop_new() };
        EVENTLOOPS.with(|el| {
            el.borrow_mut().insert(id, cb);
        });
        EventLoop { id: id }
    }

    pub fn request_animation_frame(&mut self) {
        unsafe { event_loop_raf(self.id); }
    }

    pub fn shutdown(&mut self) {
        EVENTLOOPS.with(|el| {
            el.borrow_mut().remove(&self.id);
        });
        unsafe { event_loop_shutdown(self.id); }
    }
}
