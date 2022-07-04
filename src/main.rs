use rust_boy::bus::Bus;
use rust_boy::cartridge::*;
use rust_boy::cpu::*;
use rust_boy::mbc::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please input rom file path as args 1");
        return;
    }

    let bytes = std::fs::read(&args[1]).unwrap();

    let wraped_cart = Cartridge::new(&bytes);
    let cart = wraped_cart.unwrap();

    let bus = Bus::new(new_mbc(cart));

    let cpu = Cpu::new(bus);
    cpu.step();
}
