use bitvec::prelude::*;
use image::RgbaImage;
use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::{constant::*, interrupt::Interrupt, memory::*, traits::*, types::*, util::*};

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringData,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::HBlank
    }
}

#[derive(Default)]
pub struct Ppu {
    clock: u16,
    buf: RAM,
    bus: Option<Arc<Mutex<Box<dyn BusTrait + Send>>>>,
    lcdc: Lcdc,
    lcds: Lcds,
    scroll: Scroll,
    palette: Palette,
    image_data: RgbaImage,
    dma: Byte,
    pub dma_started: bool,
    mode: Mode,
    interrupt: Arc<Mutex<Interrupt>>,
}

impl fmt::Display for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ppu: clock:{:04X} Lcdc:{} Lcds:{} Scroll:{}",
            self.clock, self.lcdc, self.lcds, self.scroll
        )
    }
}

impl Ppu {
    pub fn new(interrupt: Arc<Mutex<Interrupt>>) -> Self {
        let image_data = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

        Self {
            clock: 0,
            buf: RAM::new(0xFFFF),
            bus: None,
            dma: 0x00,
            dma_started: false,
            interrupt,
            image_data,
            ..Default::default()
        }
    }

    pub fn init(&mut self, bus: Arc<Mutex<Box<dyn BusTrait + Send>>>) {
        self.bus = Option::Some(bus);
    }

    pub fn step(&mut self, cycle: u16) {
        // println!("{}", self);
        self.clock = self.clock.wrapping_add(cycle);

        if !self.lcdc.lcd_ppu_enable {
            return;
        }

        if self.clock >= CYCLE_PER_LINE {
            if self.scroll.is_v_blank_start() {
                self.mode = Mode::HBlank; // tmp settings!!
                self.draw_sprite();
                self.interrupt.lock().unwrap().request(INT_VBLANK_FLG);
                self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
            } else if self.scroll.is_v_blank_period() {
                self.mode = Mode::VBlank;
            } else if self.scroll.is_h_blank_period() {
                self.mode = Mode::HBlank; // tmp settings!!
                self.draw_bg_line();

                if self.lcdc.window_enable {
                    self.draw_win_line();
                }
            } else {
                self.mode = Mode::HBlank; // tmp settings!!
                self.scroll.ly = 0;
                self.draw_bg_line();
            }

            if self.lcds.lyc_interrupt_enable {
                self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
            }
            self.scroll.ly = self.scroll.ly.wrapping_add(1);
            self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
            self.clock = self.clock.wrapping_sub(CYCLE_PER_LINE);
        }
    }

    fn draw_bg_line(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.image_data
                .put_pixel(x as u32, self.scroll.ly as u32, self.get_bg_tile_color(x))
        }
    }
    fn draw_win_line(&mut self) {
        if (self.scroll.wx < 0 || 167 <= self.scroll.wx)
            || (self.scroll.wy < 0 || 144 <= self.scroll.wy)
        {
            return;
        }
        if self.scroll.ly < self.scroll.wy {
            return;
        }
        for x in 0..SCREEN_WIDTH {
            self.image_data
                .put_pixel(x as u32, self.scroll.ly as u32, self.get_win_tile_color(x));
        }
    }

    // not implemented
    fn draw_sprite(&mut self) {
        for i in 0..SPRITE_NUM {
            let mut bytes4: [Byte; 4] = [0; 4];
            for j in 0..4 {
                let addr = ADDR_OAM_START
                    .wrapping_add(i.wrapping_mul(4))
                    .wrapping_add(j);
                bytes4[j as usize] = self.bus.as_ref().unwrap().lock().unwrap().read(addr);
            }
            let s = Sprite::new(&bytes4);
            // TODO long sprite
            let obj_height;
            if self.lcdc.obj_size {
                obj_height = 16;
            } else {
                obj_height = 8;
            }
            for x in 0..8 {
                for y in 0..obj_height {
                    let mut offset_x = x;
                    let mut offset_y = y;
                    let mut tile_x = x;
                    let mut tile_y = y;

                    let mut x_pos = s.x.saturating_add(offset_x);
                    let mut y_pos = s.y.saturating_add(offset_y);
                    let block: u16;
                    let tile_idx: Byte;
                    if s.tile_idx >= 128 {
                        block = 1;
                        tile_idx = s.tile_idx - 128;
                    } else {
                        block = 0;
                        tile_idx = s.tile_idx;
                    }
                    if s.y_flip() {
                        tile_y = obj_height - 1 - y;
                    }
                    if s.x_flip() {
                        tile_x = 7 - x;
                    }
                    // ignore out of screen
                    if (SCREEN_WIDTH <= x_pos) || (SCREEN_HEIGHT <= y_pos) {
                        continue;
                    }
                    let tile_color_base_addr = (0x8000 as Word)
                        .wrapping_add((block as Word).wrapping_mul(128).wrapping_mul(16))
                        .wrapping_add((tile_idx as Word).wrapping_mul(16))
                        .wrapping_add((tile_y as Word).wrapping_mul(2));

                    let lower = self
                        .bus
                        .as_ref()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .read(tile_color_base_addr);
                    let upper = self
                        .bus
                        .as_ref()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .read(tile_color_base_addr + 1);
                    let lb = bit(&lower, &(7 - tile_x));
                    let ub = bit(&upper, &(7 - tile_x));

                    let color = (ub << 1) + lb;
                    if color != 0 {
                        let c = self.palette.get_obj_palette(color, s.mgb_palette_no());
                        self.image_data.put_pixel(x_pos as u32, y_pos as u32, c);
                    }
                }
            }
        }
    }

    fn get_bg_tile_color(&self, lx: u8) -> image::Rgba<u8> {
        // yPos is current pixel from top(0-255)
        let y_pos = self.scroll.ly.wrapping_add(self.scroll.scy);
        let x_pos = lx.wrapping_add(self.scroll.scx);
        let base_addr = if self.lcdc.bg_tile_map_area {BG_TILE_MAP_AREA_1} else {BG_TILE_MAP_AREA_0};

        self.get_tile_color(x_pos, y_pos, base_addr)
    }

    fn get_win_tile_color(&self, lx: u8) -> image::Rgba<u8> {
        // yPos is current pixel from top(0-255)
        let y_pos = self.scroll.ly.wrapping_sub(self.scroll.wy);
        let x_pos = lx.wrapping_sub(self.scroll.wx.wrapping_sub(7));
        let base_addr = if self.lcdc.window_tile_map_area {WINDOW_TILE_MAP_AREA_1} else {WINDOW_TILE_MAP_AREA_0};
        self.get_tile_color(x_pos, y_pos, base_addr)
    }

    fn get_tile_color(&self, x_pos: u8, y_pos: u8, base_addr: Word) -> image::Rgba<u8> {
        let addr = Tile::get_tile_addr(y_pos, x_pos, base_addr);
        let mut tile_idx = self.bus.as_ref().unwrap().lock().unwrap().read(addr) as i8 as i32;

        let mut block: usize = 0;
        if !self.lcdc.bg_window_tile_data_area {
            if tile_idx < 0 {
                block = 1;
                tile_idx += 128;
            } else {
                block = 2;
            }
        } else {
            if tile_idx < 128 {
                block = 0;
            } else {
                block = 1;
                tile_idx -= 128;
            }
        }

        let tile_color_base_addr = (0x8000 as Word)
            .wrapping_add((block as Word).wrapping_mul(128).wrapping_mul(16))
            .wrapping_add((tile_idx as Word).wrapping_mul(16))
            .wrapping_add(((y_pos % 8) as Word).wrapping_mul(2));

        let lower = self
            .bus
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .read(tile_color_base_addr);
        let upper = self
            .bus
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .read(tile_color_base_addr + 1);
        let lb = bit(&lower, &(7 - (x_pos % 8)));
        let ub = bit(&upper, &(7 - (x_pos % 8)));

        let color = (ub << 1) + lb;
        self.palette.get_palette(color)
    }

    pub fn transfer_oam(&mut self) {
        let addr = self.dma as Word * 0x100;
        for i in 0..0xA0 {
            let b = self
                .bus
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .read(addr + i as Word);
            self.bus
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .write(ADDR_OAM_START + i as Word, b);
        }

        self.dma_started = false;
    }

    pub fn display(&self) -> image::RgbaImage {
        self.image_data.clone()
    }
}

impl Reader for Ppu {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_PPU_LCDC =>  {
                let mut value: Byte = 0;
                let v = value.view_bits_mut::<Lsb0>();
                v.set(7, self.lcdc.lcd_ppu_enable);
                v.set(6, self.lcdc.window_tile_map_area);
                v.set(5, self.lcdc.window_enable);
                v.set(4, self.lcdc.bg_window_tile_data_area);
                v.set(3, self.lcdc.bg_tile_map_area);
                v.set(2, self.lcdc.obj_size);
                v.set(1, self.lcdc.obj_enable);
                v.set(0, self.lcdc.bg_window_enable);
                value
            },
            ADDR_PPU_LCDS => {
                let mut value: Byte = 0;
                let v = value.view_bits_mut::<Lsb0>();
                v.set(7, true);
                v.set(6, self.lcds.lyc_interrupt_enable);
                v.set(5, self.lcds.oam_interrupt_enable);
                v.set(4, self.lcds.vblank_interrupt_enable);
                v.set(3, self.lcds.hblank_interrupt_enable);
                v.set(2, self.scroll.lyc == self.scroll.ly);
                v.set(1, (self.mode as u8 & 0b10) == 0b10);
                v.set(0, (self.mode as u8 & 0b01) == 0b01);
                value
            },
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
            ADDR_PPU_LCDC => {
                // TODO
                let v = value.view_bits::<Lsb0>();

                if self.lcdc.lcd_ppu_enable && !v[7] && self.mode != Mode::VBlank {
                    log::warn!("Stopping LCD operation (Bit 7 from 1 to 0) may be performed during VBlank ONLY");
                }
                self.lcdc.lcd_ppu_enable = v[7];
                self.lcdc.window_tile_map_area = v[6];
                self.lcdc.window_enable = v[5];
                self.lcdc.bg_window_tile_data_area = v[4];
                self.lcdc.bg_tile_map_area = v[3];
                self.lcdc.obj_size = v[2];
                self.lcdc.obj_enable = v[1];
                self.lcdc.bg_window_enable = v[0];
            },
            ADDR_PPU_LCDS => {
                let v = value.view_bits::<Lsb0>();
                self.lcds.lyc_interrupt_enable = v[6];
                self.lcds.oam_interrupt_enable = v[5];
                self.lcds.vblank_interrupt_enable = v[4];
                self.lcds.hblank_interrupt_enable = v[3];
            },
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

impl fmt::Display for Scroll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sroll: scy:{:02X} scx:{:02X} ly:{:02X} lyc:{:02X} wy:{:02X} wx:{:02X}",
            self.scy, self.scx, self.ly, self.lyc, self.wy, self.wx,
        )
    }
}

impl Scroll {
    fn is_v_blank_period(&self) -> bool {
        SCREEN_HEIGHT <= self.ly && self.ly <= 153
    }

    fn is_h_blank_period(&self) -> bool {
        self.ly < SCREEN_HEIGHT
    }

    fn is_v_blank_start(&self) -> bool {
        self.ly == SCREEN_HEIGHT
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

#[derive(Debug)]
struct Lcdc {
    /// Bit 7  
    /// LCD and PPU enable  
    /// false=Off, true=On  
    pub lcd_ppu_enable: bool,

    /// Bit 6  
    /// Window tile map area  
    /// false=9800-9BFF, true=9C00-9FFF  
    pub window_tile_map_area: bool,

    /// Bit 5  
    pub window_enable: bool,

    /// Bit 4  
    /// BG and Window tile data area  
    /// false=8800-97FF, true=8000-8FFF  
    pub bg_window_tile_data_area: bool,

    /// Bit 3  
    /// BG tile map area  
    /// false=9800-9BFF, true=9C00-9FFF  
    pub bg_tile_map_area: bool,
    
    /// Bit 2  
    /// OBJ size  
    /// false=8x8, true=8x16  
    pub obj_size: bool,

    /// Bit 1  
    pub obj_enable: bool,

    /// Bit 0  
    pub bg_window_enable: bool,
}

impl fmt::Display for Lcdc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lcdc: {:?}",
            self
        )
    }
}

impl Default for Lcdc {
    fn default() -> Self {
        Self { 
            lcd_ppu_enable: true,
            window_tile_map_area: false,
            window_enable: false,
            bg_window_tile_data_area: true,
            bg_tile_map_area: false,
            obj_size: false,
            obj_enable: false,
            bg_window_enable: true,
        }
    }
}

#[derive(Default, Debug)]
struct Lcds {
    pub lyc_interrupt_enable: bool,
    pub oam_interrupt_enable: bool,
    pub vblank_interrupt_enable: bool,
    pub hblank_interrupt_enable: bool,
}

impl fmt::Display for Lcds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lcds: {:?}",
            self
        )
    }
}

enum PaletteEnum {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl PaletteEnum {
    pub fn get_color(palette: PaletteEnum) -> image::Rgba<u8> {
        match palette {
            PaletteEnum::White => image::Rgba([175, 197, 160, 255]),
            PaletteEnum::LightGray => image::Rgba([93, 147, 66, 255]),
            PaletteEnum::DarkGray => image::Rgba([22, 63, 48, 255]),
            PaletteEnum::Black => image::Rgba([0, 40, 0, 255]),
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
    fn get_palette(&self, idx: u8) -> image::Rgba<u8> {
        let color: Byte = (self.bgp >> (idx * 2)) & 0x03;
        PaletteEnum::get_color(PaletteEnum::from_u8(color))
    }

    fn get_obj_palette(&self, idx: u8, obp: u8) -> image::Rgba<u8> {
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
struct Tile;

impl Tile {
    pub fn get_tile_addr(y_pos: Byte, x_pos: Byte, base_addr: Word) -> Word {
        // https://gbdev.io/pandocs/pixel_fifo.html#get-tile

        // yTile is Tile corresponding at yPos
        let y_tile: Word = y_pos as Word / 8;
        // xPos is current pixel from left(0-31)
        let x_tile: Word = x_pos as Word / 8;

        base_addr + y_tile * 32 + x_tile
    }
}

struct Sprite {
    y: Byte,
    x: Byte,
    tile_idx: Byte,
    attr: Byte,
}

impl Sprite {
    pub fn new(bytes4: &[Byte]) -> Self {
        Self {
            y: bytes4[0].wrapping_sub(16),
            x: bytes4[1].wrapping_sub(8),
            tile_idx: bytes4[2],
            attr: bytes4[3],
        }
    }

    pub fn y_flip(&self) -> bool {
        bit(&self.attr, &6) == 1
    }

    pub fn x_flip(&self) -> bool {
        bit(&self.attr, &5) == 1
    }

    pub fn mgb_palette_no(&self) -> Byte {
        bit(&self.attr, &4)
    }

    pub fn vram_bank(&self) -> bool {
        bit(&self.attr, &3) == 1
    }

    pub fn cgb_palette_no(&self) -> Byte {
        self.attr & 0x07
    }
}
