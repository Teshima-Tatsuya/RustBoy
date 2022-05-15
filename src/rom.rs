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
    cartridge_type: Byte,
    rom_size: Byte,
    ram_size: Byte,
    destination_code: Byte,
    old_licensee_code: Byte,
    mask_rom_version_number: Byte,
    header_checksum: Byte,
    global_checksum: [Byte; 2],

    data: Vec<Byte>,
}

#[derive(Default, Clone)]
struct CartridgeType {
    code: Byte,
    mbc: Mbc,
    has_ram: bool,
    has_battery: bool,
    has_timer: bool,
    has_runble: bool,
    has_sensor: bool,
}

impl CartridgeType {
    fn new(code: Byte) -> Self {
        let ct = Default::default();

        ct
    }
}

#[derive(Clone)]
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

        Ok(Rom {
            entry_point,
            logo,
            title,
            new_licensee_code,
            sgb_flag,
        })
    }
}
