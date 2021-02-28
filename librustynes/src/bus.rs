use crate::mem::Mem;
use log::*;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

#[derive(Debug)]
pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
    }
}

impl Mem for Bus {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                // let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("PPU not supported yet");
            }
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
            _ => {
                debug!("Ignoring memory write at {:#X}", addr);
            }
        }
    }
}
