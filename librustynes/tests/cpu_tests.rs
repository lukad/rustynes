// use librustynes::cpu::*;

// macro_rules! load_and_run {
//     ($prog:expr) => {{
//         let mut cpu = CPU::new();
//         cpu.load_and_run($prog);
//         cpu
//     }};
// }

// macro_rules! load_and_reset {
//     ($prog:expr) => {{
//         let mut cpu = CPU::new();
//         cpu.load($prog);
//         cpu.reset();
//         cpu
//     }};
// }

// #[test]
// fn test_nop() {
//     load_and_run!(&[0xEA, 0x00]);
// }

// #[test]
// fn test_lda_from_memory() {
//     let mut cpu = CPU::new();
//     cpu.write_byte(0x10, 0x55);
//     cpu.load_and_run(&[0xA5, 0x10, 0x00]);
//     assert_eq!(cpu.register_a, 0x55);
// }

// #[test]
// fn test_lda_immediate_byte() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xA9, 0x05, 0x00]);
//     assert_eq!(cpu.status.zero, false);
//     assert_eq!(cpu.status.negative, false);
// }

// #[test]
// fn test_lda_zero_flag() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xA9, 0x00, 0x00]);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_lda_negative_flag() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xA9, 0b1000_0001, 0x00]);
//     assert_eq!(cpu.status.negative, true);
// }

// #[test]
// fn test_tax() {
//     let mut cpu = load_and_reset!(&[0xAA, 0x00]);
//     cpu.register_a = 10;
//     cpu.run();
//     assert_eq!(cpu.register_x, 10)
// }

// #[test]
// fn test_tay() {
//     let mut cpu = load_and_reset!(&[0xA8, 0x00]);
//     cpu.register_a = 10;
//     cpu.run();
//     assert_eq!(cpu.register_y, 10)
// }

// #[test]
// fn test_tsx() {
//     let mut cpu = load_and_reset!(&[0xBA, 0x00]);
//     cpu.sp = 10;
//     cpu.run();
//     assert_eq!(cpu.register_x, 10)
// }

// #[test]
// fn test_txa() {
//     let mut cpu = load_and_reset!(&[0x8A, 0x00]);
//     cpu.register_x = 10;
//     cpu.run();
//     assert_eq!(cpu.register_a, 10)
// }

// #[test]
// fn test_txs() {
//     let mut cpu = load_and_reset!(&[0x9A, 0x00]);
//     cpu.register_x = 10;
//     cpu.run();
//     assert_eq!(cpu.sp, 10)
// }

// #[test]
// fn test_tya() {
//     let mut cpu = load_and_reset!(&[0x98, 0x00]);
//     cpu.register_y = 10;
//     cpu.run();
//     assert_eq!(cpu.register_a, 10)
// }

// #[test]
// fn test_5_ops_working_together() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xA9, 0xC0, 0xAA, 0xe8, 0x00]);

//     assert_eq!(cpu.register_x, 0xc1)
// }

// #[test]
// fn test_inx_overflow() {
//     let mut cpu = load_and_reset!(&[0xE8, 0xE8, 0x00]);
//     cpu.register_x = 0xFF;
//     cpu.run();
//     assert_eq!(cpu.register_x, 1)
// }

// #[test]
// fn test_inx() {
//     let mut cpu = load_and_reset!(&[0xE8, 0x00]);
//     cpu.register_x = 0x29;
//     cpu.run();
//     assert_eq!(cpu.register_x, 0x2A)
// }

// #[test]
// fn test_iny() {
//     let mut cpu = load_and_reset!(&[0xC8, 0x00]);
//     cpu.register_y = 0x29;
//     cpu.run();
//     assert_eq!(cpu.register_y, 0x2A)
// }

// #[test]
// fn test_dex_overflow() {
//     let cpu = load_and_run!(&[0xCA, 0x00]);
//     assert_eq!(cpu.register_x, 0xFF)
// }

// #[test]
// fn test_dex() {
//     let mut cpu = load_and_reset!(&[0xCA, 0x00]);
//     cpu.register_x = 0x2B;
//     cpu.run();
//     assert_eq!(cpu.register_x, 0x2A)
// }

// #[test]
// fn test_dey() {
//     let mut cpu = load_and_reset!(&[0x88, 0x00]);
//     cpu.register_y = 0x2B;
//     cpu.run();
//     assert_eq!(cpu.register_y, 0x2A)
// }

// #[test]
// fn test_and_immediate() {
//     let cpu = load_and_run!(&[0xA9, 0b0000_1111, 0x29, 0b1010_1010, 0x00]);
//     assert_eq!(cpu.register_a, 0b000_1010);
//     assert_eq!(cpu.status.zero, false);
//     assert_eq!(cpu.status.negative, false);
// }

// #[test]
// fn test_bcc() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0x90, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x00);
//     cpu.reset();
//     cpu.status.carry = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2A);
// }

// #[test]
// fn test_bcs() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xB0, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x2A);
//     cpu.reset();
//     cpu.status.carry = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x00);
// }

// #[test]
// fn test_beq() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(&[0xF0, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x2A);
//     cpu.reset();
//     cpu.status.zero = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x00);
// }

// #[test]
// fn test_bmi() {
//     let mut cpu = load_and_run!(&[0x30, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x2A);
//     cpu.reset();
//     cpu.status.negative = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x00);
// }

// #[test]
// fn test_bne() {
//     let mut cpu = load_and_run!(&[0xD0, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x00);
//     cpu.reset();
//     cpu.status.zero = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2A);
// }

// #[test]
// fn test_bpl() {
//     let mut cpu = load_and_run!(&[0x10, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x00);
//     cpu.reset();
//     cpu.status.negative = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2A);
// }

// #[test]
// fn test_bvc() {
//     let mut cpu = load_and_run!(&[0x50, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x00);
//     cpu.reset();
//     cpu.status.overflow = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2A);
// }

// #[test]
// fn test_bvs() {
//     let mut cpu = load_and_run!(&[0x70, 0x02, 0xA9, 0x2A, 0x00]);
//     assert_eq!(cpu.register_a, 0x2A);
//     cpu.reset();
//     cpu.status.overflow = true;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x00);
// }

// #[test]
// fn test_bit_zero_page() {
//     let mut cpu = CPU::new();
//     cpu.write_byte(0xF0u8 as u16, 0b1010_1010);
//     cpu.load(&[0x24, 0xF0]);
//     cpu.reset();
//     cpu.register_a = 0b1111_0000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0b1010_0000);
// }

// #[test]
// fn test_bit_absolute() {
//     let mut cpu = CPU::new();
//     cpu.write_byte(0x2000, 0b1010_1010);
//     cpu.load(&[0x2C, 0x00, 0x20, 0x00]);
//     cpu.reset();
//     cpu.register_a = 0b1111_0000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0b1010_0000);
// }

// #[test]
// fn test_clc() {
//     let mut cpu = CPU::new();
//     cpu.load(&[0x18, 0x00]);
//     cpu.reset();
//     cpu.status.carry = true;
//     cpu.run();
//     assert_eq!(cpu.status.carry, false);
// }

// #[test]
// fn test_sec() {
//     let cpu = load_and_run!(&[0x38, 0x00]);
//     assert_eq!(cpu.status.carry, true);
// }

// #[test]
// fn test_cld() {
//     let mut cpu = CPU::new();
//     cpu.load(&[0xD8, 0x00]);
//     cpu.reset();
//     cpu.status.decimal = true;
//     cpu.run();
//     assert_eq!(cpu.status.decimal, false);
// }

// #[test]
// fn test_sed() {
//     let cpu = load_and_run!(&[0xF8, 0x00]);
//     assert_eq!(cpu.status.decimal, true);
// }

// #[test]
// fn test_cli() {
//     let mut cpu = load_and_reset!(&[0x58, 0x00]);
//     cpu.status.disable_interrupts = true;
//     cpu.run();
//     assert_eq!(cpu.status.disable_interrupts, false);
// }

// #[test]
// fn test_sei() {
//     let cpu = load_and_run!(&[0x78, 0x00]);
//     assert_eq!(cpu.status.disable_interrupts, true);
// }

// #[test]
// fn test_sta_zero_page() {
//     let mut cpu = load_and_reset!(&[0x85, 0xFF, 0x00]);
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xFF), 0x2A);
// }

// #[test]
// fn test_sta_zero_page_x() {
//     let mut cpu = load_and_reset!(&[0x95, 0x80, 0x00]);
//     cpu.register_x = 0x0F;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x8F), 0x2A);
// }

// #[test]
// fn test_sta_zero_page_x_overflow() {
//     let mut cpu = load_and_reset!(&[0x95, 0x80, 0x00]);
//     cpu.register_x = 0x8F;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x0F), 0x2A);
// }

// #[test]
// fn test_sta_absolute() {
//     let mut cpu = load_and_reset!(&[0x8D, 0xFE, 0xF0, 0x00]);
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF0FE), 0x2A);
// }

// #[test]
// fn test_sta_absolute_x() {
//     let mut cpu = load_and_reset!(&[0x9D, 0xFE, 0xF0, 0x00]);
//     cpu.register_x = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_absolute_x_overflow() {
//     let mut cpu = load_and_reset!(&[0x9D, 0xFE, 0xF0, 0x00]);
//     cpu.register_x = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_absolute_y() {
//     let mut cpu = load_and_reset!(&[0x99, 0xFE, 0xF0, 0x00]);
//     cpu.register_y = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_absolute_y_overflow() {
//     let mut cpu = load_and_reset!(&[0x99, 0xFE, 0xF0, 0x00]);
//     cpu.register_y = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_x() {
//     let mut cpu = load_and_reset!(&[0x81, 0x40, 0x00]);
//     cpu.register_x = 0x08;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x48, 0xF10E);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_x_base_overflow() {
//     let mut cpu = load_and_reset!(&[0x81, 0x40, 0x00]);
//     cpu.register_x = 0xF0;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x30, 0xF10E);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_x_ptr_overflow() {
//     let mut cpu = load_and_reset!(&[0x81, 0xFE, 0x00]);
//     cpu.register_x = 0x01;
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xFF, 0x0E);
//     cpu.write_byte(0x00, 0xF1);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_y() {
//     let mut cpu = load_and_reset!(&[0x91, 0x40, 0x00]);
//     cpu.register_y = 0x08;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x48, 0xF10E);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_y_base_overflow() {
//     let mut cpu = load_and_reset!(&[0x91, 0x40, 0x00]);
//     cpu.register_y = 0xF0;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x30, 0xF10E);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_sta_indirect_y_ptr_overflow() {
//     let mut cpu = load_and_reset!(&[0x91, 0xFE, 0x00]);
//     cpu.register_y = 0x01;
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xFF, 0x0E);
//     cpu.write_byte(0x00, 0xF1);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF10E), 0x2A);
// }

// #[test]
// fn test_stx_zero_page() {
//     let mut cpu = load_and_reset!(&[0x86, 0xFF, 0x00]);
//     cpu.register_x = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xFF), 0x2A);
// }

// #[test]
// fn test_stx_zero_page_y() {
//     let mut cpu = load_and_reset!(&[0x96, 0x80, 0x00]);
//     cpu.register_y = 0x0F;
//     cpu.register_x = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x8F), 0x2A);
// }

// #[test]
// fn test_stx_zero_page_y_overflow() {
//     let mut cpu = load_and_reset!(&[0x96, 0x80, 0x00]);
//     cpu.register_y = 0x8F;
//     cpu.register_x = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x0F), 0x2A);
// }

// #[test]
// fn test_stx_absolute() {
//     let mut cpu = load_and_reset!(&[0x8E, 0xFE, 0xF0, 0x00]);
//     cpu.register_x = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF0FE), 0x2A);
// }

// #[test]
// fn test_sty_zero_page() {
//     let mut cpu = load_and_reset!(&[0x84, 0xFF, 0x00]);
//     cpu.register_y = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xFF), 0x2A);
// }

// #[test]
// fn test_sty_zero_page_x() {
//     let mut cpu = load_and_reset!(&[0x94, 0x80, 0x00]);
//     cpu.register_x = 0x0F;
//     cpu.register_y = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x8F), 0x2A);
// }

// #[test]
// fn test_sty_zero_page_x_overflow() {
//     let mut cpu = load_and_reset!(&[0x94, 0x80, 0x00]);
//     cpu.register_x = 0x8F;
//     cpu.register_y = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0x0F), 0x2A);
// }

// #[test]
// fn test_sty_absolute() {
//     let mut cpu = load_and_reset!(&[0x8C, 0xFE, 0xF0, 0x00]);
//     cpu.register_y = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xF0FE), 0x2A);
// }

// #[test]
// fn test_cmp_immediate_equal() {
//     let mut cpu = load_and_reset!(&[0xC9, 0x2A]);
//     cpu.register_a = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_immediate_greater() {
//     let mut cpu = load_and_reset!(&[0xC9, 0x2A]);
//     cpu.register_a = 0x2B;
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, false);
// }

// #[test]
// fn test_cmp_immediate_less() {
//     let mut cpu = load_and_reset!(&[0xC9, 0x2A]);
//     cpu.register_a = 0x29;
//     cpu.run();
//     assert_eq!(cpu.status.carry, false);
//     assert_eq!(cpu.status.zero, false);
// }

// #[test]
// fn test_cmp_zero_page() {
//     let mut cpu = load_and_reset!(&[0xC5, 0xFF, 0x00]);
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xFF, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_zero_page_x() {
//     let mut cpu = load_and_reset!(&[0xD5, 0x80, 0x00]);
//     cpu.register_x = 0x0F;
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0x8F, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_absolute() {
//     let mut cpu = load_and_reset!(&[0xCD, 0xFE, 0xF0, 0x00]);
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xF0FE, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_absolute_x() {
//     let mut cpu = load_and_reset!(&[0xDD, 0xFE, 0xF0, 0x00]);
//     cpu.register_x = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xF10E, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_absolute_y() {
//     let mut cpu = load_and_reset!(&[0xD9, 0xFE, 0xF0, 0x00]);
//     cpu.register_y = 0x10;
//     cpu.register_a = 0x2A;
//     cpu.write_byte(0xF10E, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_indirect_x() {
//     let mut cpu = load_and_reset!(&[0xC1, 0x40, 0x00]);
//     cpu.register_x = 0x08;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x48, 0xFFFF);
//     cpu.write_byte(0xFFFF, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cmp_indirect_y() {
//     let mut cpu = load_and_reset!(&[0xD1, 0x40, 0x00]);
//     cpu.register_y = 0x08;
//     cpu.register_a = 0x2A;
//     cpu.write_word(0x48, 0xFFFF);
//     cpu.write_byte(0xFFFF, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpx_immediate() {
//     let mut cpu = load_and_reset!(&[0xE0, 0x2A]);
//     cpu.register_x = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpx_zero_page() {
//     let mut cpu = load_and_reset!(&[0xE4, 0xFF, 0x00]);
//     cpu.register_x = 0x2A;
//     cpu.write_byte(0xFF, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpx_absolute() {
//     let mut cpu = load_and_reset!(&[0xEC, 0xFE, 0xF0, 0x00]);
//     cpu.register_x = 0x2A;
//     cpu.write_byte(0xF0FE, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpy_immediate() {
//     let mut cpu = load_and_reset!(&[0xC0, 0x2A]);
//     cpu.register_y = 0x2A;
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpy_zero_page() {
//     let mut cpu = load_and_reset!(&[0xC4, 0xFF, 0x00]);
//     cpu.register_y = 0x2A;
//     cpu.write_byte(0xFF, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_cpy_absolute() {
//     let mut cpu = load_and_reset!(&[0xCC, 0xFE, 0xF0, 0x00]);
//     cpu.register_y = 0x2A;
//     cpu.write_byte(0xF0FE, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.status.carry, true);
//     assert_eq!(cpu.status.zero, true);
// }

// #[test]
// fn test_dec_zero_page() {
//     let mut cpu = load_and_reset!(&[0xC6, 0xF0, 0x00]);
//     cpu.write_byte(0xF0, 0x2A);
//     cpu.run();
//     assert_eq!(cpu.read_byte(0xFF), 0x00);
// }
