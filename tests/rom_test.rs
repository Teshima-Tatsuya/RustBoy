
extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::{traits::*, gameboy::GameBoy};
use speculate::speculate;
use std::env;

fn rom_test(folder: &String, file: &String, frame: u64, pass_str: String) {
    let pwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let rom_path: String = "/tests/roms/".to_string();
    let path = pwd.to_string() + &rom_path + &folder + "/" + &file + ".gb";
    let bytes = std::fs::read(path).unwrap();

    let mut result: String = "".to_string();

    let mut gb = GameBoy::new(&bytes);
    for _ in 1..=frame {
        gb.step();
        if gb.cpu.bus.lock().unwrap().read(0xFF02) == 0xFF {
            result += &char::from_u32(gb.cpu.bus.lock().unwrap().read(0xFF01) as u32).unwrap().to_string();
            gb.cpu.bus.lock().unwrap().write(0xFF02, 0x00);
        }
        if result.contains(&pass_str){
             break;
        }
    }

    debug_assert!(result.contains(&pass_str), "{}",result);
}

fn rom_test_with_image(folder: &String, file: &String, frame: u64) {
    let pwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let rom_path: String = "/tests/roms/".to_string();
    let actual_path: String = "/tests/actual/".to_string();
    let path = pwd.to_string() + &rom_path + &folder + "/" + &file + ".gb";
    let actual_image_folder: String = pwd.to_string() + &actual_path + &folder;
    let actual_image_file: String = actual_image_folder.to_string() + "/" + &file + ".jpg";
    let bytes = std::fs::read(path).unwrap();

    let mut gb = GameBoy::new(&bytes);
    for _ in 1..=frame {
        gb.exec_frame();
    }

    let image = gb.display();

    std::fs::create_dir_all(&actual_image_folder).unwrap();
    image.save(&actual_image_file).unwrap();
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
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "01-special".to_string(), frame: 2000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "02-interrupts".to_string(), frame: 8000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "03-op sp,hl".to_string(), frame:  8000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "04-op r,imm".to_string(), frame: 4000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "05-op rp".to_string(), frame: 4000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "06-ldr,r".to_string(), frame: 400000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "07-jr,jp,call,ret,rst".to_string(), frame: 4000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "08-misc instrs".to_string(), frame: 4000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "09-op r,r".to_string(), frame: 8000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "10-bit ops".to_string(), frame: 8000000}),
                case(Args{folder: "blargg/cpu_instrs/individual".to_string(), file: "11-op a,(hl)".to_string(), frame: 8000000}),
            )]
            fn test(arg: Args) {
                rom_test(&arg.folder, &arg.file, arg.frame, "Passed".to_string());
            }
        }
        describe "cpu_instrs_with_image" {
            struct Args {
                folder: String,
                file: String,
                frame: u64,
            }
            #[rstest(arg,
                case(Args{folder: "blargg/cpu_instrs".to_string(), file: "cpu_instrs".to_string(), frame: 3200}),
            )]
            fn test(arg: Args) {
                rom_test_with_image(&arg.folder, &arg.file, arg.frame);
            }
        }
    }
}