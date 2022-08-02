use rust_boy::gameboy::GameBoy;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please input rom file path as args 1");
        return;
    }

    let bytes = std::fs::read(&args[1]).unwrap();

    let mut gb = GameBoy::new(&bytes);
    loop {
        gb.step();
    }
}
