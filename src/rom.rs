use crate::types::*;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Default)]
pub struct Rom {
    entry_point: [Byte; 4],
    logo: [Byte; 30],
    /// 0x013F-0x0142 Manufacturer Code
    /// 0x0143        CGB Flag
    title: String,
    new_licensee_code: [Byte; 2],
    sgb_flag: bool,
    cartridge_type: CartridgeType,
    rom_size: u64,
    ram_size: u64,
    destination_code: Byte,
    old_licensee_code: Byte,
    mask_rom_version_number: Byte,
    header_checksum: Byte,
    global_checksum: [Byte; 2],

    data: Vec<Byte>,
}

#[derive(Default)]
struct CartridgeType {
    code: Byte,
    mbc: Option<Mbc>,
    has_ram: bool,
    has_battery: bool,
    has_timer: bool,
    has_rumble: bool,
    has_sensor: bool,
}

impl CartridgeType {
    fn new(code: Byte) -> Result<Self> {
        let ct = Self {
            code,
            ..Default::default()
        };

        use Mbc::*;
        match code {
            0x00 => ct,
            0x01 => ct.with_mbc(Mbc1),
            0x02 => ct.with_mbc(Mbc1).with_ram(),
            0x03 => ct.with_mbc(Mbc1).with_ram().with_battery(),
            0x05 => ct.with_mbc(Mbc2),
            0x06 => ct.with_mbc(Mbc2).with_battery(),
            0x08 => ct.with_ram(),
            0x09 => ct.with_ram().with_battery(),
            0x0B => ct.with_mbc(Mmm01),
            0x0C => ct.with_mbc(Mmm01).with_ram(),
            0x0D => ct.with_mbc(Mmm01).with_ram().with_battery(),
            0x0F => ct.with_mbc(Mbc3).with_timer().with_battery(),
            0x10 => ct.with_mbc(Mbc3).with_timer().with_ram().with_battery(),
            0x11 => ct.with_mbc(Mbc3),
            0x12 => ct.with_mbc(Mbc3).with_ram(),
            0x13 => ct.with_mbc(Mbc3).with_ram().with_battery(),
            0x19 => ct.with_mbc(Mbc5),
            0x1A => ct.with_mbc(Mbc5).with_ram(),
            0x1B => ct.with_mbc(Mbc5).with_ram().with_battery(),
            0x1C => ct.with_mbc(Mbc5).with_rumble(),
            0x1D => ct.with_mbc(Mbc5).with_rumble().with_ram(),
            0x1E => ct.with_mbc(Mbc5).with_rumble().with_ram().with_battery(),
            0x20 => ct.with_mbc(Mbc6),
            0x22 => ct
                .with_mbc(Mbc7)
                .with_sensor()
                .with_rumble()
                .with_ram()
                .with_battery(),
            0xFE => ct.with_mbc(HuC3),
            0xFF => ct.with_mbc(HuC1).with_ram().with_battery(),
            v => bail!("Unsupported Cartridge Type ${v:02X}"),
        };

        Ok(ct)
    }

    fn with_mbc(mut self, mbc: Mbc) -> Self {
        self.mbc = Some(mbc);
        self
    }

    fn with_ram(mut self) -> Self {
        self.has_ram = true;
        self
    }

    fn with_battery(mut self) -> Self {
        self.has_battery = true;
        self
    }

    fn with_timer(mut self) -> Self {
        self.has_timer = true;
        self
    }

    fn with_rumble(mut self) -> Self {
        self.has_rumble = true;
        self
    }

    fn with_sensor(mut self) -> Self {
        self.has_sensor = true;
        self
    }
}

enum Mbc {
    NoMbc,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc4,
    Mbc5,
    Mbc6,
    Mbc7,
    HuC1,
    HuC3,
    Mmm01,
}

impl Rom {
    pub fn new(buf: &[Byte]) -> Result<Self> {
        let entry_point = buf[0x100..=0x103].try_into()?;
        let logo = buf[0x104..=0x133].try_into()?;
        let title = String::from_utf8_lossy(&buf[0x13f..=0x142]).to_string();
        let new_licensee_code = buf[0x144..=0x145].try_into()?;
        let sgb_flag = match buf[0x146] {
            0x00 => false,
            0x03 => true,
            v => bail!("Invalid SGB flag: ${v:02X}"),
        };
        let cartridge_type = CartridgeType::new(buf[0x0147]).unwrap();
        let rom_size = match buf[0x0148] {
            0x00 => 2 * 16 * 1024,
            0x01 => 4 * 16 * 1024,
            0x02 => 8 * 16 * 1024,
            0x03 => 16 * 16 * 1024,
            0x04 => 32 * 16 * 1024,
            0x05 => 64 * 16 * 1024,
            0x06 => 128 * 16 * 1024,
            0x07 => 256 * 16 * 1024,
            0x08 => 512 * 16 * 1024,
            0x52 => 72 * 16 * 1024,
            0x53 => 80 * 16 * 1024,
            0x54 => 96 * 16 * 1024,
        };

        let ram_size = match buf[0x0149] {
            0x00 => 0,
            0x01 => 0,
            0x02 => 1 * 8 * 1024,
            0x03 => 4 * 8 * 1024,
            0x04 => 16 * 8 * 1024,
            0x05 => 8 * 8 * 1024,
        };

        Ok(Rom {
            entry_point,
            logo,
            title,
            new_licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
        })
    }
}
