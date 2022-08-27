use super::mock::*;
use rust_boy::cpu::*;
use rust_boy::timer::Timer;
use rust_boy::interrupt::Interrupt;
use rust_boy::traits::*;
use std::{
    cell::RefCell,
    rc::Rc,
};


pub fn setup_cpu() -> Cpu {
    let bus = Rc::new(RefCell::new(MockBus::new()));
    let interrupt = Rc::new(RefCell::new(Interrupt::new()));
    let timer = Rc::new(RefCell::new(Timer::new(Rc::clone(&interrupt))));

    let cpu = Cpu::new(Rc::clone(&bus), Rc::clone(&interrupt));
    cpu
}
