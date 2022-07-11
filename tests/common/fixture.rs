use super::mock::*;
use rust_boy::cpu::*;
use rust_boy::traits::*;

pub fn setup_cpu() -> Cpu {
    let bus = MockBus::new();
    Cpu {
        reg: Register::default(),
        bus: Box::new(bus),
    }
}
