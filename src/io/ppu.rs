use crate::{constant::*, memory::*, traits::*, types::*, util::*};
use bevy::prelude::Color;

#[derive(Default)]
pub struct Ppu {
    clock: u16,
    buf: RAM,
    lcdc: Lcdc,
    lcds: Lcds,
    scroll: Scroll,
    palette: Palette,
    dma: Byte,
    dma_started: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            clock: 0,
            buf: RAM::new(0xFFFF),
            lcdc: Default::default(),
            lcds: Default::default(),
            scroll: Default::default(),
            palette: Default::default(),
            dma: 0x00,
            dma_started: false,
        }
    }

    pub fn step(&mut self, cycle: u16) {
        self.clock += cycle;

        if !self.lcdc.LcdPpuEnable() {
            return
        }

        if self.clock >= CYCLE_PER_LINE {
            if self.scroll.isVBlankStart() {

            } else if self.scroll.isVBlankPeriod() {

            } else if self.scroll.isHBlankPeriod() {

            } else {
                self.scroll.ly = 0;
            }

            self.scroll.ly += 1;
            self.clock -= CYCLE_PER_LINE;
        }
    }
}

impl Reader for Ppu {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_PPU_LCDC => self.lcdc.buf,
            ADDR_PPU_LCDS => self.lcds.buf,
            ADDR_PPU_SCY..=ADDR_PPU_LYC | ADDR_PPU_WY | ADDR_PPU_WX => self.scroll.read(addr),
            ADDR_PPU_BGP..=ADDR_PPU_OBP1 | ADDR_PPU_BCPS..=ADDR_PPU_OCPD => self.palette.read(addr),
            v => self.buf.read(addr),
        }
    }
}

impl Writer for Ppu {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_PPU_LCDC => self.lcdc.buf = value,
            ADDR_PPU_LCDS => self.lcds.buf = value,
            ADDR_PPU_SCY..=ADDR_PPU_LYC | ADDR_PPU_WY | ADDR_PPU_WX => self.scroll.write(addr, value),
            ADDR_PPU_BGP..=ADDR_PPU_OBP1 | ADDR_PPU_BCPS..=ADDR_PPU_OCPD => self.palette.write(addr, value),
            v => self.buf.write(addr, value),
        }
    }
}

#[derive(Default)]
struct Scroll {
    scy: Byte,
    scx: Byte,
    ly: Byte,
    lyc: Byte,
    wx: Byte,
    wy: Byte,
}

impl Scroll {
    fn isVBlankPeriod(&self) -> bool {
        if SCREEN_HEIGHT <= self.ly && self.ly <= 153 {
            return true
        }
    
        return false
    }
    
    fn isHBlankPeriod(&self) -> bool {
        if self.ly < SCREEN_HEIGHT {
            return true
        }
    
        return false
    }
    
    fn isVBlankStart(&self) -> bool {
        return self.ly == SCREEN_HEIGHT
    }
}
    
impl Reader for Scroll {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_PPU_SCY => self.scy,
            ADDR_PPU_SCX => self.scx,
            ADDR_PPU_LY => self.ly,
            ADDR_PPU_LYC => self.lyc,
            ADDR_PPU_WX => self.wx,
            ADDR_PPU_WY => self.wy,
            v => unreachable!("cannot read {:04X} for PPU Scroll", v),
        }
    }
}

impl Writer for Scroll {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_PPU_SCY => self.scy = value,
            ADDR_PPU_SCX => self.scx = value,
            ADDR_PPU_LY => self.ly = value,
            ADDR_PPU_LYC => self.lyc = value,
            ADDR_PPU_WX => self.wx = value,
            ADDR_PPU_WY => self.wy = value,
            v => unreachable!("cannot write {:04X} for PPU Scroll", v),
        }
    }
}

struct Lcdc {
    pub buf: Byte,
}

impl Lcdc {
    // LCD and PPU enable
    fn LcdPpuEnable(&self) -> bool {
        bit(&self.buf, &7) == 1
    }

    /*
    Window tile map area
     0=9800-9BFF, 1=9C00-9FFF
    */
    fn WinTileMapArea(&self) -> Word {
        let area = bit(&self.buf, &6);
        if area == 0 {
            return WindowTileMapArea0;
        }

        return WindowTileMapArea1;
    }

    // Window enable
    fn WindowEnable(&self) -> bool {
        bit(&self.buf, &5) == 1
    }

    /*
    BG and Window tile data area
     0=8800-97FF, 1=8000-8FFF
    */
    fn BgWinTileDataArea(&self) -> Word {
        let area = bit(&self.buf, &4);
        if area == 0 {
            return BGWindowTileDataArea0;
        }

        return BGWindowTileDataArea1;
    }

    /*
    BG tile map area
     0=9800-9BFF, 1=9C00-9FFF
    */
    fn BgTileMapArea(&self) -> Word {
        let area = bit(&self.buf, &3);
        if area == 0 {
            return BGTileMapArea0;
        }

        return BGTileMapArea1;
    }

    /*
    OBJ size
     0=8x8, 1=8x16
    */
    fn ObjSize(&self) -> u8 {
        bit(&self.buf, &2)
    }

    // OBJ enable
    fn ObjEnable(&self) -> bool {
        bit(&self.buf, &1) == 1
    }

    // BG and Window enable/priority
    fn BgWinEnable(&self) -> bool {
        bit(&self.buf, &0) == 1
    }
}

impl Default for Lcdc {
    fn default() -> Self {
        Self { buf: 0x91 }
    }
}

#[derive(Default)]
struct Lcds {
    pub buf: Byte,
}

enum PaletteEnum {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl PaletteEnum {
    pub fn get_color(palette: PaletteEnum) -> Color {
        match palette {
            PaletteEnum::White => Color::rgb(175f32, 197f32, 160f32),
            PaletteEnum::LightGray => Color::rgb(93f32, 147f32, 66f32),
            PaletteEnum::DarkGray => Color::rgb(22f32, 63f32, 48f32),
            PaletteEnum::Black => Color::rgb(0f32, 40f32, 0f32),
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PaletteEnum::White,
            1 => PaletteEnum::LightGray,
            2 => PaletteEnum::DarkGray,
            3 => PaletteEnum::Black,
            v => unreachable!("Cannot convert from {} to PaletteEnum", v),
        }
    }
}

#[derive(Default)]
struct Palette {
	// FF47
	bgp: Byte,
	// FF48
	obp0: Byte,
	// FF49
	obp1: Byte,

	// CGB Only
	// FF68
	bcps: Byte,
	// FF69
	bcpd: Byte,
	// FF6A
	ocps: Byte,
	// FF6B
	ocpd: Byte,
}

impl Palette {
    fn get_palette(&self, idx: u8) -> Color {
        let color: Byte = (self.bgp >> (idx * 2)) & 0x03;
        PaletteEnum::get_color(PaletteEnum::from_u8(color))
    }

    fn get_obj_palette(&self, idx: u8, obp: u8) -> Color {
        let color: Byte;
        if obp == 1 {
          color = (self.obp1 >> (idx * 2)) & 0x03;
        } else {
          color = (self.obp0 >> (idx * 2)) & 0x03;
        }

        PaletteEnum::get_color(PaletteEnum::from_u8(color))
    }
}

impl Reader for Palette {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_PPU_BGP => self.bgp,
            ADDR_PPU_OBP0 => self.obp0,
            ADDR_PPU_OBP1 => self.obp1,
            ADDR_PPU_BCPS => self.bcps,
            ADDR_PPU_BCPD => self.bcpd,
            ADDR_PPU_OCPS => self.ocps,
            ADDR_PPU_OCPD => self.ocpd,
            v => unreachable!("cannot read {:04X} for PPU Palette", v),
        }
    }
}

impl Writer for Palette {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_PPU_BGP => self.bgp = value,
            ADDR_PPU_OBP0 => self.obp0 = value,
            ADDR_PPU_OBP1 => self.obp1 = value,
            ADDR_PPU_BCPS => self.bcps = value,
            ADDR_PPU_BCPD => self.bcpd = value,
            ADDR_PPU_OCPS => self.ocps = value,
            ADDR_PPU_OCPD => self.ocpd = value,
            v => unreachable!("cannot write {:04X} for PPU Palette", v),
        }
    }
}
