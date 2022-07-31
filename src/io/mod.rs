mod serial;

use crate::{
    constant::*,
    traits::*,
};

pub struct IO {
    serial: serial::Serial,

}

impl IO {
    pub fn new() -> Self {
        IO {
            serial: serial::Serial::new()
        }
    }
}