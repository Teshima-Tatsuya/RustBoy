use crate::memory::{RAM, ROM};
use crate::types::*;
use anyhow::{bail, Result};
use std::fmt;

pub struct Cartridge {
    pub entry_point: [Byte; 4],
    pub logo: [Byte; 0x30],
    /// 0x013F-0x0142 Manufacturer Code
    /// 0x0143        CGB Flag
    pub title: String,
    pub new_licensee_code: [Byte; 2],
    pub sgb_flag: bool,
    pub cartridge_type: CartridgeType,
    pub rom_size: u64,
    pub ram_size: u64,
    pub destination_code: DestinationCode,
    pub old_licensee_code: Byte,
    pub mask_rom_version_number: Byte,
    pub header_checksum: Byte,
    pub global_checksum: [Byte; 2],
    pub rom: ROM,
    pub ram: RAM,
}

#[derive(Default, Clone)]
pub struct CartridgeType {
    pub code: Byte,
    pub mbc: Option<Mbc>,
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
        let ct = match code {
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

impl fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("CartridgeType:");
        result += format!(" {:02X}", self.code).as_str();

        let mbc = self.mbc.clone();
        result += format!(" {}", mbc.unwrap()).as_str();
        write!(
            f,
            "{}{}{}{}{}{}",
            &result.as_str(),
            if self.has_ram { " +RAM" } else { "" },
            if self.has_battery { " +BATTERY" } else { "" },
            if self.has_timer { " +TIMER" } else { "" },
            if self.has_rumble { " +RUNBLE" } else { "" },
            if self.has_sensor { " +SENSOR" } else { "" },
        )
    }
}

#[derive(Clone)]
pub enum Mbc {
    NoMbc,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    HuC1,
    HuC3,
    Mmm01,
}

impl fmt::Display for Mbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mbc::NoMbc => "NoMbc",
            Mbc::Mbc1 => "MBC1",
            Mbc::Mbc2 => "MBC2",
            Mbc::Mmm01 => "MMM01",
            Mbc::Mbc3 => "MBC3",
            Mbc::Mbc5 => "MBC5",
            Mbc::Mbc6 => "MBC6",
            Mbc::Mbc7 => "MBC7",
            Mbc::HuC1 => "HuC1",
            Mbc::HuC3 => "HuC3",
        };
        write!(f, "{s}")
    }
}

pub enum DestinationCode {
    Japanese,
    NonJapanese,
}

impl Default for DestinationCode {
    fn default() -> Self {
        Self::Japanese
    }
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("Cartridge:\n");
        result += format!(" title:{}\n", self.title).as_str();
        result += format!(" {}", self.cartridge_type).as_str();
        write!(f, "{}", &result.as_str())
    }
}

impl Cartridge {
    pub fn new(buf: &[Byte]) -> Result<Self> {
        let entry_point: [u8; 4] = buf[0x100..=0x103].try_into()?;
        let logo: [u8; 0x30] = buf[0x104..=0x133].try_into()?;
        let title = String::from_utf8_lossy(&buf[0x134..=0x143]).to_string();
        let new_licensee_code: [u8; 2] = buf[0x144..=0x145].try_into()?;
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
            v => bail!("Invalid Cartridge Size ${v:02X}"),
        };

        let ram_size = match buf[0x0149] {
            0x00 => 0,
            0x01 => 0,
            0x02 => 1 * 8 * 1024,
            0x03 => 4 * 8 * 1024,
            0x04 => 16 * 8 * 1024,
            0x05 => 8 * 8 * 1024,
            v => bail!("Invalid Ram Size ${v:02X}"),
        };

        let destination_code = match buf[0x014A] {
            0x00 => DestinationCode::Japanese,
            0x01 => DestinationCode::NonJapanese,
            v => bail!("Invalid Destination Code ${v:02X}"),
        };

        let old_licensee_code = buf[0x014B];
        let mask_rom_version_number = buf[0x014C];

        let header_checksum = buf[0x14D];
        let global_checksum: [u8; 2] = buf[0x014E..=0x014F].try_into()?;

        Ok(Cartridge {
            entry_point,
            logo,
            title,
            new_licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            old_licensee_code,
            mask_rom_version_number,
            header_checksum,
            global_checksum,
            rom: ROM::new(buf),
            ram: RAM::new(ram_size as usize),
        })
    }
}
