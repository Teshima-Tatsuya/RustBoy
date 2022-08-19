use crate::gameboy::GameBoy;

pub struct Emulator {
    pub gb: GameBoy,
}

impl Emulator {
    pub fn new(gb: GameBoy) -> Self {
        Self {
            gb: gb,
        }
    }
}