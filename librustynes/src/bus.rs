use crate::{mem::Mem, rom::Rom};
use log::*;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const ROM: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

#[derive(Debug)]
pub struct Bus<'a> {
    cpu_vram: [u8; 2048],
    rom: Rom<'a>,
}

impl<'a> Bus<'a> {
    pub fn new(rom: Rom<'a>) -> Self {
        Self {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    pub fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr %= 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl<'a> Mem for Bus<'a> {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                todo!("PPU not supported yet");
            }
            ROM..=ROM_END => self.read_prg_rom(addr),
            _ => {
                debug!("Ignoring memory read at {:#X}", addr);
                0
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = value;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                // let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("PPU not supported yet");
            }
            ROM..=ROM_END => {
                error!("Attempt to write to rom");
                panic!("Attempt to write to rom");
            }
            _ => {
                debug!("Ignoring memory write at {:#X}", addr);
            }
        }
    }
}
