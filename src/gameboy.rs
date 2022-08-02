use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::mbc::*;
use crate::types::*;

pub struct GameBoy {
    pub cpu: Cpu,
    // gpu: GPU,
    // apu: APU,
    // timer: Timer,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let bus = Bus::new(new_mbc(cartridge));

        let cpu = Cpu::new(Box::new(bus));

        Self { cpu }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}
