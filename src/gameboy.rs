use std::{
    cell::RefCell,
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
    ppu::Ppu,
};

pub struct GameBoy {
    pub cpu: Cpu,
    cycle: u32,
    ppu: Rc<RefCell<Ppu>>,
    // apu: APU,
    timer: Rc<RefCell<timer::Timer>>,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let timer = Rc::new(RefCell::new(timer::Timer::default()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let bus = Rc::new(RefCell::new(Bus::new(new_mbc(cartridge), Rc::clone(&timer), Rc::clone(&ppu))));
        ppu.borrow_mut().init(Rc::clone(&bus));

        let cpu = Cpu::new(Rc::clone(&bus));

        Self { 
            cpu,
            cycle: 0,
            ppu: Rc::clone(&ppu),
            timer: Rc::clone(&timer),
        }
    }

    pub fn step(&mut self) {
       // loop {
            let cycle = self.cpu.step();
            self.cycle += cycle as u32 * 4;
            self.timer.borrow_mut().tick(cycle);
            if self.timer.borrow_mut().overflow {
                self.cpu.bus.borrow_mut().interrupt().request(INT_TIMER_FLG);
                self.timer.borrow_mut().overflow = false;
            }

//            if self.cycle >= 70224 {
  //              self.cycle -= 70224;
    //            return
      //      }
       // }
    }
}
