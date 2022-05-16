use crate::traits::*;
use crate::types::*;

#[derive(Default)]
pub struct ROM {
    buf: Vec<Byte>,
}

#[derive(Default)]
pub struct RAM {
    buf: Vec<Byte>,
}

impl ROM {
    pub fn new(buf: &[Byte]) -> Self {
        ROM { buf: buf.to_vec() }
    }
}

impl RAM {
    pub fn new(buf: &[Byte]) -> Self {
        RAM { buf: buf.to_vec() }
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
