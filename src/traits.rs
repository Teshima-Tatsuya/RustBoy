use crate::types::*;

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

trait_alias!(pub trait BusTrait = Reader + Writer);
