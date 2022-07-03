use rust_boy::bus::Bus;
use rust_boy::cartridge::*;
use rust_boy::cpu::*;
use rust_boy::mbc::*;
use rust_boy::types::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = std::fs::read(&args[1]).unwrap();

    let cart = Cartridge::new(&bytes).unwrap();
    println!("{}", cart);

    let bus = Bus::new(new_mbc(cart));

    let cpu = Cpu::new(bus);
    cpu.step();
}
