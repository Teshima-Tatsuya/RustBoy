use crate::bus::Bus;
use crate::opcode::OPCODES;
use crate::traits::Reader;
use crate::types::*;
use bitvec::prelude::*;

pub struct Cpu {
    pub reg: Register,
    pub bus: Bus,
}

#[derive(Default)]
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

impl Register {
    pub fn af(&self) -> Word {
        ((self.A as Word) << 8) | (self.F.pack() as Word)
    }

    pub fn af_mut(&mut self, value: Word) {
        self.A = (value >> 8) as Byte;
        self.F.unpack((value & 0xFF) as Byte);
    }

    pub fn bc(&self) -> Word {
        ((self.B as Word) << 8) | (self.C as Word)
    }

    pub fn bc_mut(&mut self, value: Word) {
        self.B = (value >> 8) as Byte;
        self.C = (value & 0xFF) as Byte;
    }

    pub fn de(&self) -> Word {
        ((self.D as Word) << 8) | (self.E as Word)
    }

    pub fn de_mut(&mut self, value: Word) {
        self.D = (value >> 8) as Byte;
        self.E = (value & 0xFF) as Byte;
    }

    pub fn hl(&self) -> Word {
        ((self.H as Word) << 8) | (self.L as Word)
    }

    pub fn hl_mut(&mut self, value: Word) {
        self.H = (value >> 8) as Byte;
        self.L = (value & 0xFF) as Byte;
    }
}

#[derive(Default)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Flags {
    fn pack(&self) -> Byte {
        let mut data = 0;
        let v = data.view_bits_mut::<Lsb0>();
        v.set(7, self.z);
        v.set(6, self.n);
        v.set(5, self.h);
        v.set(4, self.c);

        data
    }

    fn unpack(&mut self, value: Byte) {
        let v = value.view_bits::<Lsb0>();
        self.z = v[7];
        self.n = v[6];
        self.h = v[5];
        self.c = v[4];
    }
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            bus: bus,
            reg: Register::default(),
        }
    }

    pub fn step(&self) {
        let buf = self.bus.read(0x100);
        let opcodes = &OPCODES[0];
        println!("{:?}", opcodes.code);
        println!("{}", buf);
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
