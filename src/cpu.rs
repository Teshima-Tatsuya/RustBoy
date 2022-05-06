use crate::types::*;

#[derive(Default)]
struct Cpu {
    reg: Register,
}

#[derive(Default)]
#[allow(non_snake_case)]
struct Register {
    A: Byte,
    B: Byte,
    C: Byte,
    D: Byte,
    E: Byte,
    F: Flags,
    H: Byte,
    L: Byte,
    SP: Word,
    PC: Word,
}

impl Register {
    fn af(&self) -> Word {
        100
    }

    fn bc(&self) -> Word {
        ((self.B as Word) << 8) | (self.C as Word)
    }

    fn bc_mut(&mut self, value: Word) {
        self.B = (value >> 8) as Byte;
        self.C = (value & 0xFF) as Byte;
    }

    fn de(&self) -> Word {
        ((self.D as Word) << 8) | (self.E as Word)
    }

    fn de_mut(&mut self, value: Word) {
        self.D = (value >> 8) as Byte;
        self.E = (value & 0xFF) as Byte;
    }

    fn hl(&self) -> Word {
        ((self.H as Word) << 8) | (self.L as Word)
    }

    fn hl_mut(&mut self, value: Word) {
        self.H = (value >> 8) as Byte;
        self.L = (value & 0xFF) as Byte;
    }
}

#[derive(Default)]
struct Flags {
    z: u8,
    n: u8,
    h: u8,
    c: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
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
    }
}
