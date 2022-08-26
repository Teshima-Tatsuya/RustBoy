use crate::types::*;
use crate::io::interrupt::Interrupt;
use crate::io::timer::Timer;

macro_rules! trait_alias {
    (pub trait $name:ident = $($traits:tt)+) => {
        pub trait $name: $($traits)* {}
        impl<T: $($traits)*> $name for T {}
    };
}

pub(crate) use trait_alias;

pub trait Reader {
    fn read(&self, addr: Word) -> Byte;
}

pub trait Writer {
    fn write(&mut self, addr: Word, value: Byte);
}

pub trait BusAccessor {
    fn interrupt(&mut self) -> &mut Interrupt;
}

trait_alias!(pub trait BusTrait = Reader + Writer + BusAccessor);
