use std::collections::HashMap;

use crate::cpu::AddressingMode;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref INSTRUCTION_LIST: Vec<Instruction> = vec![
        Instruction::new(0x00, "BRK", 1, 7, AddressingMode::None),
        Instruction::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute),
        Instruction::new(0x6C, "JMP", 3, 5, AddressingMode::Relative),
        Instruction::new(0x60, "RTS", 3, 6, AddressingMode::None),
        Instruction::new(0xEA, "NOP", 1, 2, AddressingMode::None),
        Instruction::new(0xAA, "TAX", 1, 2, AddressingMode::None),
        Instruction::new(0xA8, "TAY", 1, 2, AddressingMode::None),
        Instruction::new(0xBA, "TSX", 1, 2, AddressingMode::None),
        Instruction::new(0x8A, "TXA", 1, 2, AddressingMode::None),
        Instruction::new(0x9A, "TXS", 1, 2, AddressingMode::None),
        Instruction::new(0x98, "TYA", 1, 2, AddressingMode::None),
        Instruction::new(0xE8, "INX", 1, 2, AddressingMode::None),
        Instruction::new(0xC8, "INY", 1, 2, AddressingMode::None),
        Instruction::new(0xCA, "DEX", 1, 2, AddressingMode::None),
        Instruction::new(0x88, "DEY", 1, 2, AddressingMode::None),
        Instruction::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
        Instruction::new(0xFE, "INC", 3, 7, AddressingMode::AbsoluteX),
        Instruction::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
        Instruction::new(0xDE, "DEC", 3, 7, AddressingMode::AbsoluteX),
        Instruction::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x75, "ADC", 3, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x7D, "ADC", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0x79, "ADC", 2, 6, AddressingMode::AbsoluteY),
        Instruction::new(0x61, "ADC", 2, 5, AddressingMode::IndirectX),
        Instruction::new(0x71, "ADC", 2, 2, AddressingMode::IndirectY),
        Instruction::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x3D, "AND", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY),
        Instruction::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x31, "AND", 2, 5, AddressingMode::IndirectY),
        Instruction::new(0x90, "BCC", 2, 2, AddressingMode::Relative),
        Instruction::new(0xB0, "BCS", 2, 2, AddressingMode::Relative),
        Instruction::new(0xF0, "BEQ", 2, 2, AddressingMode::Relative),
        Instruction::new(0x30, "BMI", 2, 2, AddressingMode::Relative),
        Instruction::new(0xD0, "BNE", 2, 2, AddressingMode::Relative),
        Instruction::new(0x10, "BPL", 2, 2, AddressingMode::Relative),
        Instruction::new(0x50, "BVC", 2, 2, AddressingMode::Relative),
        Instruction::new(0x70, "BVS", 2, 2, AddressingMode::Relative),
        Instruction::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x18, "CLC", 1, 2, AddressingMode::None),
        Instruction::new(0x38, "SEC", 1, 2, AddressingMode::None),
        Instruction::new(0xD8, "CLD", 1, 2, AddressingMode::None),
        Instruction::new(0xF8, "SED", 1, 2, AddressingMode::None),
        Instruction::new(0x58, "CLI", 1, 2, AddressingMode::None),
        Instruction::new(0x78, "SEI", 1, 2, AddressingMode::None),
        Instruction::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xF5, "SBC", 3, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xFD, "SBC", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0xF9, "SBC", 2, 6, AddressingMode::AbsoluteY),
        Instruction::new(0xE1, "SBC", 2, 5, AddressingMode::IndirectX),
        Instruction::new(0xF1, "SBC", 2, 2, AddressingMode::IndirectY),
        Instruction::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
        Instruction::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0xB2, "LDA", 2, 2, AddressingMode::IndirectY),
        Instruction::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
        Instruction::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBE, "LDX", 3, 4, AddressingMode::AbsoluteY),
        Instruction::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBC, "LDY", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),
        Instruction::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        Instruction::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
        Instruction::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
        Instruction::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xDD, "CMP", 3, 4, AddressingMode::AbsoluteX),
        Instruction::new(0xD9, "CMP", 3, 4, AddressingMode::AbsoluteY),
        Instruction::new(0xC1, "CMP", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0xD1, "CMP", 2, 5, AddressingMode::IndirectY),
        Instruction::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x4A, "LSR", 1, 2, AddressingMode::None),
        Instruction::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x5E, "LSR", 3, 7, AddressingMode::AbsoluteX),
    ];
    pub(crate) static ref INSTRUCTIONS: HashMap<u8, &'static Instruction> = INSTRUCTION_LIST
        .iter()
        .map(|ins| (ins.opcode, ins))
        .collect();
}

#[derive(Debug)]
pub(crate) struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl Instruction {
    fn new(opcode: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            opcode,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}
