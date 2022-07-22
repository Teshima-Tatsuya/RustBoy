extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::cpu::*;
use rust_boy::opcode::*;
use rust_boy::types::*;
use speculate::speculate;

speculate! {
    describe "LD" {
            #[rstest()]
            fn test() {
                let mut cpu = common::fixture::setup_cpu();
                let value = cpu.load(&"A".to_string());
                assert_eq!(cpu.reg.A ,value);
                let value = cpu.load(&"B".to_string());
                assert_eq!(cpu.reg.B ,value);
                let value = cpu.load(&"C".to_string());
                assert_eq!(cpu.reg.C ,value);
            }
    }
}