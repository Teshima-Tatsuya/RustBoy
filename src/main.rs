use rust_boy::cpu::*;
use rust_boy::rom::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[0]);

    let rom = Rom::new(file);

    let cpu = Cpu::new();
    cpu.reg.F.z;
}
