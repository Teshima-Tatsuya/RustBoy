use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::mbc::*;
use crate::types::*;

struct GB {
    cpu: Cpu,
    // gpu: GPU,
    // apu: APU,
    // timer: Timer,
}

impl GB {
    fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let bus = Bus::new(new_mbc(cartridge));

        let cpu = Cpu::new(bus);

        Self { cpu }
    }
}
