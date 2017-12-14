use ::game::{Config};

/*
 *     The reason we don't use a simple bool for key state is that
 * it would skip a key that was pressed and released on the same frame.
 * This somehow increases the logic required to handle key presses,
 * but handles instant key presses much better.
 */
#[derive(PartialEq, Copy, Clone)]
enum KeyState {
    Up,
    Down,
    BeenDown,
}

impl KeyState {
    fn up(&mut self) {
        *self = match *self {
            KeyState::Up => KeyState::Up,
            KeyState::Down => KeyState::BeenDown,
            KeyState::BeenDown => KeyState::BeenDown,
        };
    }

    fn down(&mut self) {
        *self = KeyState::Down;
    }

    fn tick(&mut self) {
        *self = match *self {
            KeyState::Up => KeyState::Up,
            KeyState::Down => KeyState::Down,
            KeyState::BeenDown => KeyState::Up,
        };
    }

    fn been_pressed(&self) -> bool {
        *self != KeyState::Up
    }

    fn is_down(&self) -> bool {
        *self == KeyState::Down
    }
}

#[derive(Copy, Clone)]
pub enum InputIndex {
    Shoot = 0,
    Forward = 1,
    Backward = 2,
    Left = 3,
    Right = 4,
    _NumberOfInputs = 5,
}

pub struct Inputs {
    inputs: [KeyState; InputIndex::_NumberOfInputs as usize],
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            inputs: [KeyState::Up; InputIndex::_NumberOfInputs as usize],
        }
    }

    pub fn been_pressed(&self, idx: InputIndex) -> bool {
        self.inputs[idx as usize].been_pressed()
    }

    pub fn is_down(&self, idx: InputIndex) -> bool {
        self.inputs[idx as usize].is_down()
    }

    pub fn tick(&mut self) {
        for input in self.inputs.iter_mut() {
            input.tick();
        }
    }

    pub fn key_down(&mut self, code: u32, config: &Config) {
        if let Some(index) = config.lookup_input_key(code) {
            self.inputs[index as usize].down();
        }
    }

    pub fn key_up(&mut self, code: u32, config: &Config) {
        if let Some(index) = config.lookup_input_key(code) {
            self.inputs[index as usize].up();
        }
    }
}
