use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::mbc::*;
use crate::types::*;
use crate::constant::*;

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
        let cycle = self.cpu.step();
        self.cpu.bus.timer().tick(cycle * 4);
        if self.cpu.bus.timer().overflow {
            self.cpu.bus.interrupt().request(INT_TIMER_FLG);
            self.cpu.bus.timer().overflow = false;
        }
    }
}
