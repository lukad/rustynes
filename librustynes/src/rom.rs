use thiserror::Error;

const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Error, Debug, PartialEq)]
pub enum RomError {
    #[error("file does not start with iNes tag")]
    NesTagNotFound,
    #[error("version {0} not supported")]
    VersionNotSupported(u8),
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug)]
pub struct Rom<'a> {
    pub prg_rom: &'a [u8],
    pub chr_rom: &'a [u8],
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl<'a> Rom<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, RomError> {
        if data[0..4] != NES_TAG {
            return Err(RomError::NesTagNotFound);
        }

        let mapper = data[7] & 0b1111_0000 | data[6] >> 4;
        let ines_version = (data[7] >> 2) & 0b11;
        if ines_version != 0 {
            return Err(RomError::VersionNotSupported(ines_version));
        }

        let four_screen = data[6] & 0b1000 != 0;
        let vertical_mirroring = data[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = data[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = data[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = data[6] & 0b0100 != 0;
        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Self {
            prg_rom: &data[prg_rom_start..(prg_rom_start + prg_rom_size)],
            chr_rom: &data[chr_rom_start..(chr_rom_start + chr_rom_size)],
            mapper,
            screen_mirroring,
        })
    }
}
