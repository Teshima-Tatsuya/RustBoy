use image::RgbaImage;
use std::sync::{Arc, Mutex};

use crate::{constant::*, interrupt::Interrupt, memory::*, traits::*, types::*, util::*};

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
    bus: Option<Arc<Mutex<Box<dyn BusTrait + Send>>>>,
    lcdc: Lcdc,
    lcds: Lcds,
    scroll: Scroll,
    palette: Palette,
    image_data: RgbaImage,
    dma: Byte,
    pub dma_started: bool,
    interrupt: Arc<Mutex<Interrupt>>,
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
        self.clock = self.clock.wrapping_add(cycle);

        if !self.lcdc.lcd_ppu_enable() {
            return;
        }

        if self.clock >= CYCLE_PER_LINE {
            if self.scroll.is_v_blank_start() {
                self.draw_sprite();
                self.interrupt.lock().unwrap().request(INT_VBLANK_FLG);
                self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
            } else if self.scroll.is_v_blank_period() {
                self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
            } else if self.scroll.is_h_blank_period() {
                self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
                self.draw_bg_line();

                if self.lcdc.window_enable() {
                    self.draw_win_line();
                }
            } else {
                self.scroll.ly = 0;
                self.draw_bg_line();
            }

            if self.scroll.ly == self.scroll.scy {
                self.lcds.buf |= 0x04;
                if self.lcds.lyc() {
                    self.interrupt.lock().unwrap().request(INT_LCD_STAT_FLG);
                }
            } else {
                self.lcds.buf &= 0xFB;
            }
            self.scroll.ly = self.scroll.ly.wrapping_add(1);
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
            let obj_height: u8 = 8;
            // if g.LCDC.OBJSize() == 1 {
            // 	objHeight = 16
            // } else {
            // 	objHeight = 8
            // }
            for x in 0..8 {
                for y in 0..obj_height {
                    let mut offset_x = x;
                    let mut offset_y = y;

                    let mut x_pos = s.x.saturating_add(offset_x);
                    let mut y_pos = s.y.saturating_add(offset_y);
                    // ignore out of screen
                    if (x_pos < 0 || SCREEN_WIDTH <= x_pos) || (y_pos < 0 || SCREEN_HEIGHT <= y_pos)
                    {
                        continue;
                    }
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
                        offset_y = 7 - y;
                    }
                    y_pos = s.y + offset_y;
                    if s.x_flip() {
                        offset_x = 7 - x
                    }
                    x_pos = s.x + offset_x;
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
        let base_addr = self.lcdc.bg_tile_map_area();

        self.get_tile_color(x_pos, y_pos, base_addr)
    }

    fn get_win_tile_color(&self, lx: u8) -> image::Rgba<u8> {
        // yPos is current pixel from top(0-255)
        let y_pos = self.scroll.ly.wrapping_sub(self.scroll.wy);
        let x_pos = lx.wrapping_sub(self.scroll.wx.wrapping_sub(7));
        let base_addr = self.lcdc.win_tile_map_area();
        self.get_tile_color(x_pos, y_pos, base_addr)
    }

    fn get_tile_color(&self, x_pos: u8, y_pos: u8, base_addr: Word) -> image::Rgba<u8> {
        let addr = Tile::get_tile_addr(y_pos, x_pos, base_addr);
        let mut tile_idx = self.bus.as_ref().unwrap().lock().unwrap().read(addr) as i8 as i32;

        let mut block: usize = 0;
        if self.lcdc.bg_win_tile_data_area() == BG_WINDOW_TILE_DATA_AREA_0 {
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
            ADDR_PPU_LCDC => self.lcdc.buf,
            ADDR_PPU_LCDS => self.lcds.buf | 0x80,
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
            ADDR_PPU_LCDS => self.lcds.buf = value & 0x7F,
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

struct Lcdc {
    pub buf: Byte,
}

impl Lcdc {
    // LCD and PPU enable
    fn lcd_ppu_enable(&self) -> bool {
        bit(&self.buf, &7) == 1
    }

    /*
    Window tile map area
     0=9800-9BFF, 1=9C00-9FFF
    */
    fn win_tile_map_area(&self) -> Word {
        let area = bit(&self.buf, &6);
        if area == 0 {
            return WINDOW_TILE_MAP_AREA_0;
        }

        return WINDOW_TILE_MAP_AREA_1;
    }

    // Window enable
    fn window_enable(&self) -> bool {
        bit(&self.buf, &5) == 1
    }

    /*
    BG and Window tile data area
     0=8800-97FF, 1=8000-8FFF
    */
    fn bg_win_tile_data_area(&self) -> Word {
        let area = bit(&self.buf, &4);
        if area == 0 {
            return BG_WINDOW_TILE_DATA_AREA_0;
        }

        return BG_WINDOW_TILE_DATA_AREA_1;
    }

    /*
    BG tile map area
     0=9800-9BFF, 1=9C00-9FFF
    */
    fn bg_tile_map_area(&self) -> Word {
        let area = bit(&self.buf, &3);
        if area == 0 {
            return BG_TILE_MAP_AREA_0;
        }

        return BG_TILE_MAP_AREA_1;
    }

    /*
    OBJ size
     0=8x8, 1=8x16
    */
    fn obj_size(&self) -> u8 {
        bit(&self.buf, &2)
    }

    // OBJ enable
    fn obj_enable(&self) -> bool {
        bit(&self.buf, &1) == 1
    }

    // BG and Window enable/priority
    fn bg_win_enable(&self) -> bool {
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
    pub fn lyc(&self) -> bool {
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
