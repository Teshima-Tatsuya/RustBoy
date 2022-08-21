use std::{
    cell::Cell,
    rc::Rc,
};

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::Cpu,
    mbc::*,
    types::*,
    constant::*,
    io::*,
};

pub struct GameBoy {
    pub cpu: Cpu,
    cycle: u32,
    // gpu: GPU,
    // apu: APU,
    timer: Rc<Cell<timer::Timer>>,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let timer = Rc::new(Cell::new(timer::Timer::default()));
        let bus = Bus::new(new_mbc(cartridge));

        let cpu = Cpu::new(Box::new(bus));

        Self { 
            cpu,
            cycle: 0,
            timer: timer.clone(),
        }
    }

    pub fn step(&mut self) {
       // loop {
            let cycle = self.cpu.step();
            self.cycle += cycle as u32 * 4;
            self.cpu.bus.timer().tick(cycle);
            if self.cpu.bus.timer().overflow {
                self.cpu.bus.interrupt().request(INT_TIMER_FLG);
                self.cpu.bus.timer().overflow = false;
            }

//            if self.cycle >= 70224 {
  //              self.cycle -= 70224;
    //            return
      //      }
       // }
    }
}
