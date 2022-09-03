use super::mock::*;
use rust_boy::cpu::*;
use rust_boy::timer::Timer;
use rust_boy::interrupt::Interrupt;
use rust_boy::traits::*;
use std::{
    cell::RefCell,
    sync::Arc,
};


pub fn setup_cpu() -> Cpu {
    let bus = Arc::new(RefCell::new(MockBus::new()));
    let interrupt = Arc::new(RefCell::new(Interrupt::new()));
    let timer = Arc::new(RefCell::new(Timer::new(Arc::clone(&interrupt))));

    let cpu = Cpu::new(Arc::clone(&bus), Arc::clone(&interrupt));
    cpu
}
