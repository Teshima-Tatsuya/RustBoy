extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::{traits::*, gameboy::GameBoy};
use speculate::speculate;
use std::env;

fn rom_test(folder: String, file: String, frame: u64, pass_str: String) {
    let pwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let rom_path: String = "/tests/roms/".to_string();
    let path = pwd.to_string() + &rom_path + &folder + "/" + &file + ".gb";
    let bytes = std::fs::read(path).unwrap();

    let mut result: String = "".to_string();

    let mut gb = GameBoy::new(&bytes);
    for _ in 1..=frame {
        gb.step();
        if gb.cpu.bus.read(0xFF02) == 0xFF {
            result += &char::from_u32(gb.cpu.bus.read(0xFF01) as u32).unwrap().to_string();
            gb.cpu.bus.write(0xFF02, 0x00);
        }
        if result.contains(&pass_str){
             break;
        }
    }

    debug_assert!(result.contains(&pass_str), "{}",result);
}

speculate! {
    describe "Blargg" {
        describe "cpu_instrs" {
                struct Args {
                    folder: String,
                    file: String,
                    frame: u64,
                }
                #[rstest(arg,
                    case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "03-op sp,hl".to_string(), frame: 200}),
                )]
                fn test(arg: Args) {
                    rom_test(arg.folder, arg.file, arg.frame, "Passed".to_string());
                }
            }
    }
}