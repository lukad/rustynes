use std::{thread::sleep, time::Duration};

use librustynes::{Mem, Rom, CPU};

use log::*;
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    EventPump,
};

fn handle_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.write_byte(0xFF, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.write_byte(0xFF, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.write_byte(0xFF, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.write_byte(0xFF, 0x64);
            }
            _ => (),
        }
    }
}

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GRAY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x0600 {
        let color_idx = cpu.read_byte(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let rom_data = std::fs::read(std::env::args().nth(1).expect("Expected arg"))?;
    let rom = Rom::new(&rom_data)?;
    let mut cpu = CPU::new(rom);
    cpu.reset();
    cpu.pc = 0xC000;

    debug!("Creating SDL2 context");
    let context = sdl2::init().unwrap();

    debug!("Initializing video subsystem");
    let video_subsystem = context.video().unwrap();

    debug!("Creating window");
    let window = video_subsystem
        .window("meh", 320, 320)
        .position_centered()
        .build()
        .unwrap();

    debug!("Creating canvas");
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    debug!("Creating event pump");
    let mut event_pump = context.event_pump().unwrap();

    debug!("Creating screen texture");
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let mut screen_state = [0u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    debug!("Starting cpu");
    cpu.run_with_callback(move |cpu| {
        println!("{}", cpu.trace());

        handle_input(cpu, &mut event_pump);
        cpu.write_byte(0xFE, rng.gen_range(0..16));

        if read_screen_state(cpu, &mut screen_state) {
            let _ = texture.update(None, &screen_state, 32 * 3);
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        sleep(Duration::new(0, 70_000));
    });

    Ok(())
}
