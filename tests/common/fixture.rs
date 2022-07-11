use super::mock::*;
use rust_boy::cpu::Cpu;

pub fn setup_cpu() -> Cpu {
    let bus = MockBus::new();
    Cpu {
        bus: bus as rust_boy::bus::Bus,
    }
}
