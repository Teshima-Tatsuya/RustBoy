use crate::{constant::*, traits::*, types::*, util::*};

pub struct Joypad {
	p1: Byte,
	state: Byte,
}

pub const BUTTON_A: Byte = 0x01;
pub const BUTTON_B: Byte = 0x02;
pub const BUTTON_SELECT: Byte = 0x04;
pub const BUTTON_START: Byte = 0x08;
pub const BUTTON_RIGHT: Byte = 0x10;
pub const BUTTON_LEFT: Byte = 0x20;
pub const BUTTON_UP: Byte = 0x40;
pub const BUTTON_DOWN: Byte = 0x80;

impl Joypad {
	pub fn new() -> Self {
		Self {
			p1: 0xCF, // all buttuns are not pressed
			state: 0x00,
		}
	}

	// button is start, select, A, B
	fn button_pressed(&self) -> bool {
		bit(&self.p1, &5) == 0
	}

	// direction is up, down, left, right
	fn direction_pressed(&self) -> bool {
		bit(&self.p1, &4) == 0
	}

	pub fn press(&mut self, button: Byte) {
		self.state |= button;
	}

	pub fn release(&mut self, button: Byte) {
		self.state &= !button;
	}
}

impl Reader for Joypad {
	fn read(&self, _addr: Word) -> Byte {
		if self.button_pressed() {
			return self.p1 & !(self.state & 0x0F) | 0xC0;
		}

		if self.direction_pressed() {
			return self.p1 & !(self.state >> 4) | 0xC0;
		}

		// all not pressed
		return self.p1 | 0xCF;
	}
}

impl Writer for Joypad {
	fn write(&mut self, _addr: Word, value: Byte) {
		// because bit 3-0 is read only
		self.p1 = (self.p1 & 0xCF) | (value & 0x30);
	}
}
