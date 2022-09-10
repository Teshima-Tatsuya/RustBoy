use crate::traits::*;
use crate::types::*;

#[derive(Default)]
pub struct ROM {
    pub buf: Vec<Byte>,
}

#[derive(Default)]
pub struct RAM {
    buf: Vec<Byte>,
}

impl ROM {
    pub fn new(buf: &[Byte]) -> Self {
        Self { buf: buf.to_vec() }
    }
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self { buf: vec![0; size] }
    }
}

impl Reader for ROM {
    fn read(&self, addr: Word) -> Byte {
        self.buf[addr as usize]
    }
}

impl Reader for RAM {
    fn read(&self, addr: Word) -> Byte {
        self.buf[addr as usize]
    }
}

impl Writer for RAM {
    fn write(&mut self, addr: Word, value: Byte) {
        self.buf[addr as usize] = value;
    }
}
