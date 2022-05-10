use crate::types::*;

#[derive(Default)]
pub struct Rom {
    entry_point: [Byte; 4],
    logo: [Byte; 30],
    /// 0x013F-0x0142 Manufacturer Code
    /// 0x0143        CGB Flag
    title: [Byte; 10],
    new_licence_code: [Byte; 2],
    sgb_flag: [Byte; 2],
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

impl Rom {
    pub fn new(buf: Vec<Byte>) -> Self {
        Self::default()
    }
}
