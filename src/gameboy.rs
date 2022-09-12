use std::{sync::{Arc, Mutex}};

use crate::{
    bus::Bus, cartridge::Cartridge, cpu::Cpu, interrupt::Interrupt, mbc::*,
    ppu::Ppu, timer::Timer, types::*, joypad::Joypad,
};

pub struct GameBoy {
    pub cpu: Cpu,
    cycle: u32,
    ppu: Arc<Mutex<Ppu>>,
    // apu: APU,
    timer: Arc<Mutex<Timer>>,
    pub joypad: Arc<Mutex<Joypad>>,
}

impl GameBoy {
    pub fn new(buf: &[Byte]) -> Self {
        let wraped_cartridge = Cartridge::new(buf);
        let cartridge = wraped_cartridge.unwrap();

        let interrupt = Arc::new(Mutex::new(Interrupt::new()));
        let joypad = Arc::new(Mutex::new(Joypad::new()));
        let ppu = Arc::new(Mutex::new(Ppu::new(Arc::clone(&interrupt))));
        let timer = Arc::new(Mutex::new(Timer::new(Arc::clone(&interrupt))));
        let bus = Arc::new(Mutex::new(Bus::new(
            new_mbc(cartridge),
            Arc::clone(&timer),
            Arc::clone(&interrupt),
            Arc::clone(&ppu),
            Arc::clone(&joypad),
        )));
        ppu.lock().unwrap().init(Arc::clone(&bus));

        let cpu = Cpu::new(Arc::clone(&bus), Arc::clone(&interrupt));

        Self {
            cpu,
            cycle: 0,
            ppu: Arc::clone(&ppu),
            timer: Arc::clone(&timer),
            joypad: Arc::clone(&joypad),
        }
    }

    pub fn step(&mut self) {
        let cycle: u16;
        if self.ppu.lock().unwrap().dma_started {
            self.ppu.lock().unwrap().transfer_oam();
            cycle = 162;
        } else {
            cycle = self.cpu.step();
        }
        self.cycle += cycle as u32 * 4;
        self.ppu.lock().unwrap().step(cycle * 4);
        self.timer.lock().unwrap().tick(cycle);
    }

    pub fn exec_frame(&mut self) {
        loop {
            self.step();
            if self.cycle >= 70224 {
                self.joypad.lock().unwrap().press(0x00);
                self.joypad.lock().unwrap().release(0x00);
                self.cycle -= 70224;
                return;
            }
        }
    }

    pub fn display(&self) -> image::RgbaImage {
        self.ppu.lock().unwrap().display()
    }
}
