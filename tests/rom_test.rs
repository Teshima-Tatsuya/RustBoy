
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

        describe "blargg" {
            struct Args {
                folder: String,
                file: String,
                frame: u64,
            }
            #[rstest(arg,
                case(Args{folder: "blargg/cpu_instrs".to_string(), file: "cpu_instrs".to_string(), frame: 3200}),
                case(Args{folder: "blargg/mem_timing".to_string(), file: "mem_timing".to_string(), frame: 2000}),
                case(Args{folder: "blargg/mem_timing/individual".to_string(), file: "01-read_timing".to_string(), frame: 2000}),
                case(Args{folder: "blargg/mem_timing/individual".to_string(), file: "02-write_timing".to_string(), frame: 2000}),
                case(Args{folder: "blargg/mem_timing/individual".to_string(), file: "03-modify_timing".to_string(), frame: 2000}),
            )]
            fn test(arg: Args) {
                rom_test_with_image(&arg.folder, &arg.file, arg.frame);
            }
        }
    }

    describe "mooneye-gb" {
        describe "acceptance" {
            struct Args {
                folder: String,
                file: String,
                frame: u64,
            }
            #[rstest(arg,
                case(Args{folder: "mooneye-gb/acceptance/bits".to_string(), file: "mem_oam".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/bits".to_string(), file: "reg_f".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/bits".to_string(), file: "unused_hwio-GS".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/instr".to_string(), file: "daa".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/interrupts".to_string(), file: "ie_push".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/oam_dma".to_string(), file: "basic".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/oam_dma".to_string(), file: "reg_read".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/oam_dma".to_string(), file: "sources-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "hblank_ly_scx_timing-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_1_2_timing-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_2_0_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_2_mode0_timing_sprites".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_2_mode0_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_2_mode3_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "intr_2_oam_ok_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "stat_irq_blocking".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "stat_lyc_onoff".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/ppu".to_string(), file: "vblank_stat_intr-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/serial".to_string(), file: "boot_sclk_align-dmgABCmgb".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "div_write".to_string(), frame: 50}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "rapid_toggle".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim00_div_trigger".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim00".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim01_div_trigger".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim01".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim10_div_trigger".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim10".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim11_div_trigger".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tim11".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tima_reload".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tima_write_reloading".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance/timer".to_string(), file: "tma_write_reloading".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "add_sp_e_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_div-dmg0".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_div-dmgABCmgb".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_div-S".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_div2-S".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_hwio-dmg0".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_hwio-dmgABCmgb".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_hwio-S".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_regs-dmg0".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_regs-dmgABC".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_regs-mgb".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_regs-sgb".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "boot_regs-sgb2".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "call_cc_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "call_cc_timing2".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "call_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "di_timing-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "div_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "ei_sequence".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "ei_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "halt_ime0_ei".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "halt_ime0_nointr_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "halt_ime1_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "halt_ime1_timing2-GS".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "if_ie_registers".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "intr_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "jp_cc_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "jp_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "ld_hl_sp_e_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "oam_dma_restart".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "oam_dma_start".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "oam_dma_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "pop_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "push_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "rapid_di_ei".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "ret_cc_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "ret_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "reti_intr_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "reti_timing".to_string(), frame: 100}),
                case(Args{folder: "mooneye-gb/acceptance".to_string(), file: "rst_timing".to_string(), frame: 100}),
            )]
            fn test(arg: Args) {
                rom_test_with_image(&arg.folder, &arg.file, arg.frame);
            }
        }

        describe "emulator-only" {
            struct Args {
                folder: String,
                file: String,
                frame: u64,
            }
            #[rstest(arg,
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "bits_bank1".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "bits_bank2".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "bits_mode".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "bits_ramg".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "multicart_rom_8Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "ram_64kb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "ram_256kb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_1Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_2Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_4Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_8Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_16Mb".to_string(), frame: 10}),
                case(Args{folder: "mooneye-gb/emulator-only/mbc1".to_string(), file: "rom_512kb".to_string(), frame: 10}),
            )]
            fn test(arg: Args) {
                rom_test_with_image(&arg.folder, &arg.file, arg.frame);
            }
        }
    }
}