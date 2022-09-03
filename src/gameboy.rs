use std::{cell::RefCell, sync::Arc};

use crate::{
    bus::Bus, cartridge::Cartridge, constant::*, cpu::Cpu, interrupt::Interrupt, io::*, mbc::*,
    ppu::Ppu, timer::Timer, types::*,
};

pub struct GameBoy {
    pub cpu: Cpu,
    cycle: u32,
    ppu: Arc<RefCell<Ppu>>,
    // apu: APU,
    timer: Arc<RefCell<Timer>>,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let interrupt = Arc::new(RefCell::new(Interrupt::new()));
        let ppu = Arc::new(RefCell::new(Ppu::new(Arc::clone(&interrupt))));
        let timer = Arc::new(RefCell::new(Timer::new(Arc::clone(&interrupt))));
        let bus = Arc::new(RefCell::new(Bus::new(
            new_mbc(cartridge),
            Arc::clone(&timer),
            Arc::clone(&interrupt),
            Arc::clone(&ppu),
        )));
        ppu.borrow_mut().init(Arc::clone(&bus));

        let cpu = Cpu::new(Arc::clone(&bus), Arc::clone(&interrupt));

        Self {
            cpu,
            cycle: 0,
            ppu: Arc::clone(&ppu),
            timer: Arc::clone(&timer),
        }
    }

    pub fn step(&mut self) {
        let cycle: u16;
        if self.ppu.borrow().dma_started {
            self.ppu.borrow_mut().transfer_oam();
            cycle = 162;
        } else {
            cycle = self.cpu.step();
        }
        self.cycle += cycle as u32 * 4;
        self.ppu.borrow_mut().step(cycle * 4);
        self.timer.borrow_mut().tick(cycle);
    }

    pub fn exec_frame(&mut self) {
        loop {
            self.step();
            if self.cycle >= 70224 {
                self.cycle -= 70224;
                return;
            }
        }
    }
}
