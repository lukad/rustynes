use crate::{bus::Bus, instruction::INSTRUCTIONS, mem::Mem, rom::Rom};

const STACK: u16 = 0x0100;

#[derive(Debug)]
pub(crate) enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Relative,
    None,
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    X,
    Y,
    S,
}

#[derive(Debug)]
pub(crate) enum BranchCondition {
    CarrySet,
    CarryClear,
    ZeroSet,
    ZeroClear,
    MinusSet,
    MinusClear,
    OverflowSet,
    OverflowClear,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StatusRegister {
    pub carry: bool,
    pub zero: bool,
    pub disable_interrupts: bool,
    pub decimal: bool,
    pub b1: bool,
    pub b2: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for StatusRegister {
    fn from(v: u8) -> Self {
        Self {
            carry: v & (1 << 0) != 0,
            zero: v & (1 << 1) != 0,
            disable_interrupts: v & (1 << 2) != 0,
            decimal: v & (1 << 3) != 0,
            b1: v & (1 << 4) != 0,
            b2: v & (1 << 5) != 0,
            overflow: v & (1 << 6) != 0,
            negative: v & (1 << 7) != 0,
        }
    }
}

impl Into<u8> for StatusRegister {
    fn into(self) -> u8 {
        let mut result = 0;
        result |= self.carry as u8;
        result |= (self.zero as u8) << 1;
        result |= (self.disable_interrupts as u8) << 2;
        result |= (self.decimal as u8) << 3;
        result |= (self.b1 as u8) << 4;
        result |= (self.b2 as u8) << 5;
        result |= (self.overflow as u8) << 6;
        result |= (self.negative as u8) << 7;
        result
    }
}

#[derive(Debug)]
pub struct CPU<'a> {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub sp: u8,
    pub status: StatusRegister,
    pub pc: u16,
    bus: Bus<'a>,
}

impl<'a> Mem for CPU<'a> {
    fn read_byte(&self, addr: u16) -> u8 {
        self.bus.read_byte(addr)
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.bus.write_byte(addr, value);
    }
}

impl<'a> CPU<'a> {
    pub fn new(rom: Rom<'a>) -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            sp: 0,
            status: Default::default(),
            pc: 0,
            bus: Bus::new(rom),
        }
    }

    pub fn trace(&self) -> String {
        let opcode = self.read_byte(self.pc);
        let instruction = INSTRUCTIONS.get(&opcode).unwrap();

        let hex_dump = ((self.pc)..(self.pc + instruction.len as u16))
            .into_iter()
            .map(|addr| format!("{:02X}", self.read_byte(addr)))
            .collect::<Vec<_>>()
            .join(" ");

        let param = match instruction.len {
            1 => match opcode {
                0x0a | 0x4a | 0x2a | 0x6a => "A".to_string(),
                _ => "".to_string(),
            },
            2 => {
                let addr = self.read_byte(self.pc + 1);
                match opcode {
                    0x20 => "".to_string(),
                    _ => match &instruction.mode {
                        AddressingMode::Immediate => format!("#${:02X}", addr),
                        AddressingMode::ZeroPage => {
                            format!("${:02X} = {:02X}", addr, self.read_byte(addr as u16))
                        }
                        AddressingMode::Relative => {
                            format!(
                                "${:04X}",
                                self.pc.wrapping_add(2).wrapping_add(addr as i8 as u16)
                            )
                        }
                        mode => todo!("{:?}", mode),
                    },
                }
            }
            3 => {
                let addr = self.read_word(self.pc + 1);
                match opcode {
                    0x20 => format!("${:04X}", addr),
                    _ => match &instruction.mode {
                        AddressingMode::None => "".to_string(),
                        AddressingMode::Absolute => match opcode {
                            0x4C => format!("${:04X}", addr),
                            _ => format!("${:04} = {:02X}", addr, self.read_byte(addr)),
                        },
                        mode => todo!("{:?}", mode),
                    },
                }
            }
            _ => unreachable!(),
        };

        let status: u8 = self.status.into();
        format!(
            "{:04X}  {:8}  {} {:<28}A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.pc,
            hex_dump,
            instruction.mnemonic,
            param,
            self.register_a,
            self.register_x,
            self.register_y,
            status,
            self.sp,
            // 0, // TODO: use ppu register once implemented
        )
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.pc = 0x0600;
        self.run();
    }

    pub fn load(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.write_byte(0x0600 + i as u16, *byte);
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.sp = 0xFD;
        self.status = 0b100100u8.into();
        self.pc = self.read_word(0xFFFC);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.read_byte(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let addr = self.read_byte(self.pc);
                addr.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = self.read_byte(self.pc);
                addr.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute => self.read_word(self.pc),
            AddressingMode::AbsoluteX => {
                let addr = self.read_word(self.pc);
                addr.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr = self.read_word(self.pc);
                addr.wrapping_add(self.register_y as u16)
            }
            AddressingMode::IndirectX => {
                let base = self.read_byte(self.pc);
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.read_byte(ptr as u16) as u16;
                let hi = self.read_byte(ptr.wrapping_add(1) as u16) as u16;
                hi << 8 | lo
            }
            AddressingMode::IndirectY => {
                let base = self.read_byte(self.pc);
                let ptr = (base as u8).wrapping_add(self.register_y);
                let lo = self.read_byte(ptr as u16) as u16;
                let hi = self.read_byte(ptr.wrapping_add(1) as u16) as u16;
                hi << 8 | lo
            }
            AddressingMode::Relative => {
                let offset = self.read_byte(self.pc) as i8;
                self.pc.wrapping_add(offset as u16)
            }
            AddressingMode::None => {
                panic!("addressing mode {:?} is not supported", mode)
            }
        }
    }

    fn get_register_value(&self, register: Register) -> u8 {
        match register {
            Register::A => self.register_a,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            Register::S => self.sp,
        }
    }

    fn set_register_value(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.register_a = value,
            Register::X => self.register_x = value,
            Register::Y => self.register_y = value,
            Register::S => self.sp = value,
        }
    }

    fn push_byte(&mut self, value: u8) {
        self.write_byte(STACK + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_byte(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read_byte(STACK + self.sp as u16)
    }

    fn push_word(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.push_byte(hi);
        self.push_byte(lo);
    }

    fn pop_word(&mut self) -> u16 {
        let lo = self.pop_byte() as u16;
        let hi = self.pop_byte() as u16;
        hi << 8 | lo
    }

    fn pla(&mut self) {
        self.register_a = self.pop_byte(); // + 0x10;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn pha(&mut self) {
        self.push_byte(self.register_a);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        self.pc = self.get_operand_address(mode);
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.push_word(self.pc.wrapping_add(1));
        self.pc = addr;
    }

    fn rts(&mut self) {
        self.pc = self.pop_word().wrapping_add(1);
    }

    fn add_to_a(&mut self, value: u8) {
        let carry = if self.status.carry { 1 } else { 0 };
        let sum = self.register_a as u16 + value as u16 + carry;

        self.status.carry = sum > 0xFF;
        let sum = sum as u8;

        self.status.overflow = (value ^ sum) & (sum ^ self.register_a) & 0x80 != 0;

        self.register_a = sum;
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.add_to_a(value);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        let value = ((value as i8).wrapping_neg().wrapping_sub(1)) as u8;
        self.add_to_a(value);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.register_a |= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn branch(&mut self, condition: BranchCondition, mode: &AddressingMode) {
        let condition_met = match condition {
            BranchCondition::CarrySet => self.status.carry,
            BranchCondition::CarryClear => !self.status.carry,
            BranchCondition::ZeroSet => self.status.zero,
            BranchCondition::ZeroClear => !self.status.zero,
            BranchCondition::MinusSet => self.status.negative,
            BranchCondition::MinusClear => !self.status.negative,
            BranchCondition::OverflowSet => self.status.overflow,
            BranchCondition::OverflowClear => !self.status.overflow,
        };
        if condition_met {
            self.pc = self.get_operand_address(mode);
        }
        self.pc += 1;
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.register_a &= value;
        let status: StatusRegister = value.into();
        self.status.overflow = status.overflow;
        self.status.negative = status.negative;
        self.status.zero = self.register_a == 0;
    }

    fn clc(&mut self) {
        self.status.carry = false
    }

    fn sec(&mut self) {
        self.status.carry = true;
    }

    fn clv(&mut self) {
        self.status.overflow = false;
    }

    fn cld(&mut self) {
        self.status.decimal = false
    }

    fn sed(&mut self) {
        self.status.decimal = true;
    }

    fn cli(&mut self) {
        self.status.disable_interrupts = false;
    }

    fn sei(&mut self) {
        self.status.disable_interrupts = true;
    }

    fn load_register(&mut self, to: Register, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        self.set_register_value(to, value);
        self.update_zero_and_negative_flags(value);
    }

    fn store(&mut self, register: Register, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.get_register_value(register);

        self.write_byte(addr, value);
    }

    fn compare(&mut self, register: Register, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr);
        let register_value = self.get_register_value(register);

        self.status.carry = value <= register_value;
        self.update_zero_and_negative_flags(register_value.wrapping_sub(value));
    }

    fn transfer(&mut self, from: Register, to: Register) {
        let value = self.get_register_value(from);
        self.set_register_value(to, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inc_register(&mut self, register: Register) {
        let value = self.get_register_value(register).wrapping_add(1);
        self.set_register_value(register, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dec_register(&mut self, register: Register) {
        let value = self.get_register_value(register).wrapping_sub(1);
        self.set_register_value(register, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr).wrapping_sub(1);
        self.write_byte(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read_byte(addr).wrapping_add(1);
        self.write_byte(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn lsr_accumulator(&mut self) {
        self.status.carry = self.register_a & 1 == 1;
        self.register_a >>= 1;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.read_byte(addr);
        self.status.carry = value & 1 == 1;
        value >>= 1;
        self.write_byte(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.status.zero = value == 0;
        self.status.negative = value & 0b1000_0000 != 0;
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {})
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            let opcode = self.read_byte(self.pc);

            let instruction = INSTRUCTIONS
                .get(&opcode)
                .unwrap_or_else(|| panic!("could not decode opcode {:#04X}", opcode));

            callback(self);

            self.pc += 1;
            let pc = self.pc;

            match opcode {
                0x00 => {
                    self.status.b1 = true;
                    return;
                }

                0x08 => self.push_byte(self.status.into()),
                0x28 => self.status = self.pop_byte().into(),

                0x48 => self.pha(),
                0x68 => self.pla(),

                0x4C | 0x6C => self.jmp(&instruction.mode),
                0x20 => self.jsr(&instruction.mode),
                0x60 => self.rts(),

                0xEA => (),

                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&instruction.mode)
                }

                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&instruction.mode)
                }

                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ora(&instruction.mode)
                }

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&instruction.mode)
                }

                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&instruction.mode)
                }

                0x18 => self.clc(),
                0x38 => self.sec(),
                0xB8 => self.clv(),
                0xD8 => self.cld(),
                0xF8 => self.sed(),
                0x58 => self.cli(),
                0x78 => self.sei(),

                0xB0 => self.branch(BranchCondition::CarrySet, &instruction.mode),
                0x90 => self.branch(BranchCondition::CarryClear, &instruction.mode),
                0xF0 => self.branch(BranchCondition::ZeroSet, &instruction.mode),
                0xD0 => self.branch(BranchCondition::ZeroClear, &instruction.mode),
                0x30 => self.branch(BranchCondition::MinusSet, &instruction.mode),
                0x10 => self.branch(BranchCondition::MinusClear, &instruction.mode),
                0x70 => self.branch(BranchCondition::OverflowSet, &instruction.mode),
                0x50 => self.branch(BranchCondition::OverflowClear, &instruction.mode),

                0x24 | 0x2c => self.bit(&instruction.mode),

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.load_register(Register::A, &instruction.mode)
                }

                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.load_register(Register::X, &instruction.mode)
                }

                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.load_register(Register::Y, &instruction.mode)
                }

                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.store(Register::A, &instruction.mode);
                }
                0x86 | 0x96 | 0x8E => self.store(Register::X, &instruction.mode),
                0x84 | 0x94 | 0x8C => self.store(Register::Y, &instruction.mode),

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.compare(Register::A, &instruction.mode)
                }
                0xE0 | 0xE4 | 0xEC => self.compare(Register::X, &instruction.mode),
                0xC0 | 0xC4 | 0xCC => self.compare(Register::Y, &instruction.mode),

                0xAA => self.transfer(Register::A, Register::X),
                0xA8 => self.transfer(Register::A, Register::Y),
                0xBA => self.transfer(Register::S, Register::X),
                0x8A => self.transfer(Register::X, Register::A),
                0x9A => self.transfer(Register::X, Register::S),
                0x98 => self.transfer(Register::Y, Register::A),

                0xE8 => self.inc_register(Register::X),
                0xC8 => self.inc_register(Register::Y),
                0xCA => self.dec_register(Register::X),
                0x88 => self.dec_register(Register::Y),

                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&instruction.mode),
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&instruction.mode),

                0x4A => self.lsr_accumulator(),
                0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&instruction.mode),

                _ => todo!(),
            }

            if pc == self.pc {
                self.pc += (instruction.len - 1) as u16;
            }
        }
    }
}
