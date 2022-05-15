use rust_boy::cpu::*;
use rust_boy::rom::*;
use rust_boy::types::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = std::fs::read(&args[0]).unwrap();

    let rom = Rom::new(&bytes);

    let cpu = Cpu::new();
    cpu.reg.F.z;
}
