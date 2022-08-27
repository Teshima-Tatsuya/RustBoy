use super::mock::*;
use rust_boy::cpu::*;
use rust_boy::traits::*;
use std::{
    cell::RefCell,
    rc::Rc,
};


pub fn setup_cpu() -> Cpu {
    let bus = MockBus::new();
    Cpu {
        reg: Register::default(),
        bus: Rc::new(RefCell::new(Box::new(bus))),
        halted: false,
        ime: false,
    }
}
