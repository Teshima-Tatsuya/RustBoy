use crate::types::*;

pub struct Mbc1 {}

impl super::MbcTrait for Mbc1 {
    fn read(&mut self, addr: Word) -> Byte {
        0
    }
}
