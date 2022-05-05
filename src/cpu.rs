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

    fn de(&self) -> Word {
        ((self.D as Word) << 8) | (self.E as Word)
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
    pub fn New() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_get() {
        let mut reg: Register = Default::default();
        assert_eq!(reg.af(), 1234);
    }
}
