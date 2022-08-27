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
    timer::Timer,
    interrupt::Interrupt,
};

pub struct GameBoy {
    pub cpu: Cpu,
    cycle: u32,
    ppu: Rc<RefCell<Ppu>>,
    // apu: APU,
    timer: Rc<RefCell<Timer>>,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let interrupt = Rc::new(RefCell::new(Interrupt::new()));
        let timer = Rc::new(RefCell::new(Timer::new(Rc::clone(&interrupt))));
        let bus = Rc::new(RefCell::new(Bus::new(new_mbc(cartridge), Rc::clone(&timer),Rc::clone(&interrupt), Rc::clone(&ppu))));
        ppu.borrow_mut().init(Rc::clone(&bus));

        let cpu = Cpu::new(Rc::clone(&bus), Rc::clone(&interrupt));

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
//            if self.cycle >= 70224 {
  //              self.cycle -= 70224;
    //            return
      //      }
       // }
    }
}
