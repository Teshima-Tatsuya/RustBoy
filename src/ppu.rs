use std::{cell::RefCell, rc::Rc};

use crate::{
    constant::*,
    interrupt::Interrupt,
    memory::*,
    traits::*,
    types::*,
    util::*
};

#[derive(Debug, Default)]
struct Color(u8, u8, u8, u8); // rgba

enum Mode {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringData,
}

#[derive(Default)]
pub struct Ppu {
    clock: u16,
    buf: RAM,
    bus: Option<Rc<RefCell<Box<dyn BusTrait>>>>,
    lcdc: Lcdc,
    lcds: Lcds,
    scroll: Scroll,
    palette: Palette,
    tiles: Vec<Vec<Tile>>,
    image_data: Vec<Vec<Color>>,
    dma: Byte,
    pub dma_started: bool,
    interrupt: Rc<RefCell<Interrupt>>,
}

impl Ppu {
    pub fn new(interrupt: Rc<RefCell<Interrupt>>) -> Self {
        Self {
            clock: 0,
            buf: RAM::new(0xFFFF),
            bus: None,
            dma: 0x00,
            dma_started: false,
            interrupt: interrupt,
            ..Default::default()
        }
    }

    pub fn init(&mut self, bus: Rc<RefCell<Box<dyn BusTrait>>>) {
        self.bus = Option::Some(bus);
    }

    pub fn step(&mut self, cycle: u16) {
        self.clock = self.clock.wrapping_add(cycle);

        if !self.lcdc.LcdPpuEnable() {
            return;
        }

        if self.clock >= CYCLE_PER_LINE {
            if self.scroll.isVBlankStart() {
                self.interrupt.borrow_mut().request(INT_VBLANK_FLG);
                if self.lcds.mode1() {
                    self.interrupt.borrow_mut().request(INT_LCD_STAT_FLG);
                }
            } else if self.scroll.isVBlankPeriod() {
            } else if self.scroll.isHBlankPeriod() {
            } else {
                self.scroll.ly = 0;
            }

            self.scroll.ly += 1;
            self.clock -= CYCLE_PER_LINE;
        }
    }

    fn load_tile(&mut self) {
        let addr = 0x8000;
        let tile_num = 128;
        let bytes16: [Byte; 16];

        for block in 0..3 {
            for i in 0..tile_num {
                for b in 0..16 {}
            }
        }
    }

    pub fn transfer_oam(&mut self) {
        for i in 0..0xA0 {
            let addr = self.dma as Word * 0x100;
            let b = self.bus.as_ref().unwrap().borrow().read(addr + i as Word);
            self.bus.as_mut().unwrap().borrow_mut().write(ADDR_OAM_START + i as Word, b);
        }

        self.dma_started = false;
    }
}

impl Reader for Ppu {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_PPU_LCDC => self.lcdc.buf,
            ADDR_PPU_LCDS => self.lcds.buf,
            ADDR_PPU_SCY..=ADDR_PPU_LYC | ADDR_PPU_WY | ADDR_PPU_WX => self.scroll.read(addr),
            ADDR_PPU_BGP..=ADDR_PPU_OBP1 | ADDR_PPU_BCPS..=ADDR_PPU_OCPD => self.palette.read(addr),
            ADDR_PPU_DMA => self.dma,
            v => self.buf.read(addr),
        }
    }
}

impl Writer for Ppu {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_PPU_LCDC => self.lcdc.buf = value,
            ADDR_PPU_LCDS => self.lcds.buf = value,
            ADDR_PPU_SCY..=ADDR_PPU_LYC | ADDR_PPU_WY | ADDR_PPU_WX => {
                self.scroll.write(addr, value)
            }
            ADDR_PPU_BGP..=ADDR_PPU_OBP1 | ADDR_PPU_BCPS..=ADDR_PPU_OCPD => {
                self.palette.write(addr, value)
            }
            ADDR_PPU_DMA => {
                self.dma_started = true;
                self.dma = value;
            }
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
            return true;
        }

        return false;
    }
    fn isHBlankPeriod(&self) -> bool {
        if self.ly < SCREEN_HEIGHT {
            return true;
        }

        return false;
    }
    fn isVBlankStart(&self) -> bool {
        return self.ly == SCREEN_HEIGHT;
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

impl Lcds {
    // Bit 6
    // LY STAT Interrupt source
    pub fn LYC(&self) -> bool {
        return bit(&self.buf, &6) == 1;
    }

    // Bit 5
    // OAM STAT Interrupt source
    pub fn mode2(&self) -> bool {
        return bit(&self.buf, &5) == 1;
    }

    // Bit 4
    // VBlank STAT Interrupt source
    pub fn mode1(&self) -> bool {
        return bit(&self.buf, &4) == 1;
    }

    // Bit 3
    // HBlank STAT Interrupt source
    pub fn mode0(&self) -> bool {
        return bit(&self.buf, &3) == 1;
    }

    // Bit 2
    // OAM STAT Interrupt source
    pub fn ly_lcy(&self) -> bool {
        return bit(&self.buf, &2) == 1;
    }
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
            PaletteEnum::White => Color(175, 197, 160, 255),
            PaletteEnum::LightGray => Color(93, 147, 66, 255),
            PaletteEnum::DarkGray => Color(22, 63, 48, 255),
            PaletteEnum::Black => Color(0, 40, 0, 255),
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

#[derive(Default, Debug)]
struct Tile {
    pub buf: Vec<Vec<Byte>>,
}

impl Tile {
    fn new(bytes16: &[Byte]) -> Self {
        let mut buf: Vec<Vec<Byte>>;
        buf = Vec::new();

        for y in 0..8 {
            let lower = bytes16[y * 2];
            let upper = bytes16[y * 2 + 1];

            let mut xs = Vec::new();

            for x in (0..8).rev() {
                let lb = bit(&lower, &x);
                let ub = bit(&upper, &x);

                let color = (ub << 1) + lb;
                xs.push(color);
            }
            buf.push(xs);
        }

        Self { buf }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_new() {
        let bytes = [
            0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97,
            0x7E, 0xFF,
        ];
        let tile = Tile::new(&bytes);

        let colors: Vec<Vec<Byte>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![2, 3, 3, 3, 3, 3, 3, 2],
            vec![3, 0, 0, 0, 0, 1, 0, 3],
            vec![3, 0, 0, 0, 1, 0, 2, 3],
            vec![3, 0, 0, 1, 0, 2, 1, 3],
            vec![3, 0, 1, 0, 2, 1, 2, 3],
            vec![3, 1, 0, 2, 1, 2, 2, 3],
            vec![2, 3, 3, 3, 3, 3, 3, 2],
        ];

        assert_eq!(colors, tile.buf);
    }
}
