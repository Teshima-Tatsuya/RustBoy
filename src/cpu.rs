use crate::{
    opcode::{OPCODES, CB_OPCODES},
    traits::*,
    types::*,
    util::*,
    bus::Bus,
    constant::*,
    io::interrupt::Interrupt,
    io::timer::Timer,
};

use bitvec::prelude::*;
use std::fmt;

trait_alias!(pub trait BusTrait = Reader + Writer + BusAccessor);

pub struct Cpu {
    pub reg: Register,
    pub bus: Box<dyn BusTrait>,
    pub halted: bool,
    pub ime: bool,
}

#[allow(non_snake_case)]
pub struct Register {
    pub A: Byte,
    pub B: Byte,
    pub C: Byte,
    pub D: Byte,
    pub E: Byte,
    pub F: Flags,
    pub H: Byte,
    pub L: Byte,
    pub SP: Word,
    pub PC: Word,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Register: SP:{:04X} PC:{:04X} A:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X}",
            self.SP, self.PC, self.A, self.B, self.C, self.D, self.E, self.H, self.L
        )
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            A: 0x01,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            F: Flags::default(),
            H: 0x01,
            L: 0x4D,
            SP: 0xFFFE,
            PC: 0x0100,
        }
    }
}
impl Register {
    pub fn r(&self, r: &String) -> Byte {
        match r.as_str() {
            "A" => self.A,
            "B" => self.B,
            "C" => self.C,
            "D" => self.D,
            "E" => self.E,
            "H" => self.H,
            "L" => self.L,
            "F" => self.F.pack(),
            _ => unreachable!(),
        }
    }

    pub fn r_mut(&mut self, r: &String, value: Byte) {
        match r.as_str() {
            "A" => self.A = value,
            "B" => self.B = value,
            "C" => self.C = value,
            "D" => self.D = value,
            "E" => self.E = value,
            "H" => self.H = value,
            "L" => self.L = value,
            "F" => self.F.unpack(value),
            _ => unreachable!(),
        }
    }

    pub fn r16(&mut self, r: &String) -> Word {
        match r.as_str() {
            "AF" => self.af(),
            "BC" => self.bc(),
            "DE" => self.de(),
            "HL" => self.hl(),
            "HLD" => {
                let res = self.hl();
                self.hl_mut(res - 1);
                res
            },
            "HLI" => {
                let res = self.hl();
                self.hl_mut(res + 1);
                res
            },
            "PC" => self.PC,
            "SP" => self.SP,
            _ => unreachable!(),
        }
    }

    pub fn r16_mut(&mut self, r16: &String, value: Word) {
        match r16.as_str() {
            "AF" => self.af_mut(value),
            "BC" => self.bc_mut(value),
            "DE" => self.de_mut(value),
            "HL" => self.hl_mut(value),
            "PC" => self.pc_mut(value),
            "SP" => self.sp_mut(value),
            _ => unreachable!(),
        }
    }

    pub fn af(&self) -> Word {
        ((self.A as Word) << 8) | (self.F.pack() as Word)
    }

    fn af_mut(&mut self, value: Word) {
        self.A = (value >> 8) as Byte;
        self.F.unpack((value & 0xFF) as Byte);
    }

    pub fn bc(&self) -> Word {
        ((self.B as Word) << 8) | (self.C as Word)
    }

    fn bc_mut(&mut self, value: Word) {
        self.B = (value >> 8) as Byte;
        self.C = (value & 0xFF) as Byte;
    }

    pub fn de(&self) -> Word {
        ((self.D as Word) << 8) | (self.E as Word)
    }

    fn de_mut(&mut self, value: Word) {
        self.D = (value >> 8) as Byte;
        self.E = (value & 0xFF) as Byte;
    }

    pub fn hl(&self) -> Word {
        ((self.H as Word) << 8) | (self.L as Word)
    }

    fn hl_mut(&mut self, value: Word) {
        self.H = (value >> 8) as Byte;
        self.L = (value & 0xFF) as Byte;
    }
    fn pc_mut(&mut self, value: Word) {
        self.PC = value;
    }
    fn sp_mut(&mut self, value: Word) {
        self.SP = value;
    }
}

pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            z: true,
            n: true,
            h: false,
            c: false,
        }
    }
}

impl Flags {
    pub fn pack(&self) -> Byte {
        let mut data = 0;
        let v = data.view_bits_mut::<Lsb0>();
        v.set(7, self.z);
        v.set(6, self.n);
        v.set(5, self.h);
        v.set(4, self.c);

        data
    }

    pub fn unpack(&mut self, value: Byte) {
        let v = value.view_bits::<Lsb0>();
        self.z = v[7];
        self.n = v[6];
        self.h = v[5];
        self.c = v[4];
    }

    pub fn f(&self, flag: String) -> bool {
        match flag.as_str() {
            "Z" => self.z,
            "N" => self.n,
            "H" => self.h,
            "C" => self.c,
            _ => unreachable!(),
        }
    }
}

impl Cpu {
    pub fn new(bus: Box<dyn BusTrait>) -> Self {
        Self {
            bus: bus,
            reg: Register::default(),
            halted: false,
            ime: false,
        }
    }

    pub fn step(&mut self) -> u16 {
        if self.halted{
            if self.bus.interrupt().has() {
                self.halted = false;
            }
            return 1;
        }

        if self.exec_interrupt() {
            return 1;
        }

        let mut opcode = self.fetch();

        let op;
        if opcode == 0xCB {
            opcode = self.fetch();
            op = &CB_OPCODES[opcode as usize];
        } else {
            op = &OPCODES[opcode as usize];
        }
        // println!(" {}", op);
        // println!(" {}", self.reg);
        // println!(
        //     "  data: {:02X}{:02X}",
        //     self.bus.read(self.reg.PC),
        //     self.bus.read(self.reg.PC + 1)
        // );

        let handler = &op.handler;
        handler(self, op.r1.to_string(), op.r2.to_string());

        return op.cycles as u16;
    }

    pub fn fetch(&mut self) -> Byte {
        let buf = self.bus.read(self.reg.PC);
        self.reg.PC += 1;
        return buf;
    }

    pub fn fetch16(&mut self) -> Word {
        let lower = self.fetch();
        let upper = self.fetch();
        return bytes_2_word(upper, lower);
    }

    pub fn push(&mut self, buf: Byte) {
        self.reg.SP -= 1;
        self.bus.write(self.reg.SP, buf)
    }

    // push PC
    pub fn push_pc(&mut self) {
        self.push(extract_upper(self.reg.PC));
        self.push(extract_lower(self.reg.PC));
    }

    pub fn pop(&mut self) -> Byte {
        let d = self.bus.read(self.reg.SP);
        self.reg.SP += 1;
        return d;
    }

    pub fn pop_pc(&mut self) {
        let lower = self.pop();
        let upper = self.pop();

        self.reg.PC = bytes_2_word(upper, lower)
    }

    pub fn load(&mut self, reg: &String) -> Word {
        match reg.as_str() {
            // r
            "A" | "B" | "C" | "D" | "E" | "F" | "H" | "L" => self.reg.r(reg) as Word,
            // m
            "(C)" => {
                let r = self.reg.C;
                self.bus.read(bytes_2_word(0xFF, r)) as Word
            },
            // d
            "d" => {
                self.fetch() as Word
            },
            "dd" => {
                self.fetch16()
            },
            // a
            "(a)" => {
                let addr = self.fetch();
                self.bus.read(bytes_2_word(0xFF as Byte, addr)) as Word
            },
            "aa" => {
                self.fetch16()
            },
            "(aa)" => {
                let addr = self.fetch16();
                self.bus.read(addr) as Word
            },
            // rr
            "AF" | "BC" | "DE" | "HL" | "HLD" | "HLI" | "PC" | "SP" => self.reg.r16(reg),
            // mm
            "(BC)" | "(DE)" | "(HL)" | "(HLI)" | "(HLD)" => {
                let mut s = reg.replace("(", "");
                s = s.replace(")", "");
                self.bus.read(self.reg.r16(&s)) as Word
            }, 
            &_ => unreachable!()
        }
    }

    pub fn store(&mut self, reg: &String, value: Word) {
        match reg.as_str() {
            // r
            "A" | "B" | "C" | "D" | "E" | "F" | "H" | "L" => self.reg.r_mut(reg, value as Byte),
            // m
            "(C)" => {
                let mut s = reg.replace("(", "");
                s = s.replace(")", "");
                let addr = bytes_2_word(0xFF, self.reg.r(&s));
                self.bus.write(addr, value as Byte);
            },
            // a
            "(a)" => {
                let addr = self.fetch();
                self.bus.write(bytes_2_word(0xFF as Byte, addr), value as Byte);
            },
            "(aa)" => {
                let addr = self.fetch16();
                self.bus.write(addr, value as Byte);
            },
            // rr
            "AF" | "BC" | "DE" | "HL" | "SP" | "PC" => self.reg.r16_mut(reg, value),
            // mm
            "(BC)" | "(DE)" | "(HL)" | "(HLI)" | "(HLD)" => {
                let mut s = reg.replace("(", "");
                s = s.replace(")", "");
                let addr = self.reg.r16(&s);
                self.bus.write(addr, value as Byte);
            }, 
            &_ => unreachable!()
        }
    }

    pub fn cond(&mut self, cond: &String) -> bool {
        match cond.as_str() {
            "Z" => self.reg.F.z,
            "NZ" => !self.reg.F.z,
            "C" => self.reg.F.c,
            "NC" => !self.reg.F.c,
            &_ => unreachable!(),
        }
    }

    fn exec_interrupt(&mut self) -> bool {
        if !self.ime || !self.interrupt().has() {
            return false
        }
    
        let addr = self.interrupt().interrupt_addr();

        match addr {
            INT_TIMER_ADDR => self.timer().overflow = false,
            _ => (),
        }

        self.push_pc();
        self.store(&"PC".to_string(), addr);
        self.ime = false;
    
        return true
    }

    pub fn interrupt(&mut self) -> &mut Interrupt {
        self.bus.interrupt()
    }

    pub fn timer(&mut self) -> &mut Timer {
        self.bus.timer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_set_get() {
        let mut reg: Register = Default::default();
        reg.bc_mut(0x1234);
        assert_eq!(reg.bc(), 0x1234);
        reg.de_mut(0x5678);
        assert_eq!(reg.de(), 0x5678);
        reg.hl_mut(0x9012);
        assert_eq!(reg.hl(), 0x9012);
        reg.af_mut(0x1357);
        assert_eq!(reg.af(), 0x1350);
    }
}
