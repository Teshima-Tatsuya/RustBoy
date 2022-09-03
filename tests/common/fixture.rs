use super::mock::*;
use rust_boy::cpu::*;
use rust_boy::timer::Timer;
use rust_boy::interrupt::Interrupt;
use rust_boy::traits::*;
use std::{
    sync::{
        Arc,
        Mutex,
    },
};


pub fn setup_cpu() -> Cpu {
    let bus = Arc::new(Mutex::new(MockBus::new()));
    let interrupt = Arc::new(Mutex::new(Interrupt::new()));
    let timer = Arc::new(Mutex::new(Timer::new(Arc::clone(&interrupt))));

    let cpu = Cpu::new(Arc::clone(&bus), Arc::clone(&interrupt));
    cpu
}
