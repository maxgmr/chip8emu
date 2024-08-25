use std::{env, fs::File, io::Read};

use chip8core::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

/// Background colour.
pub const BG_RGB: (u8, u8, u8) = (0, 0, 0);
/// Foreground colour.
pub const FG_RGB: (u8, u8, u8) = (0, 255, 0);

/// Emulator speed.
pub const TICKS_PER_FRAME: usize = 8;

/// Multiplier for screen size.
pub const SCALE: u32 = 15;

// Key bindings.
pub const KEY_1: Keycode = Keycode::Num1;
pub const KEY_2: Keycode = Keycode::Num2;
pub const KEY_3: Keycode = Keycode::Num3;
pub const KEY_C: Keycode = Keycode::Num4;

pub const KEY_4: Keycode = Keycode::Q;
pub const KEY_5: Keycode = Keycode::W;
pub const KEY_6: Keycode = Keycode::E;
pub const KEY_D: Keycode = Keycode::R;

pub const KEY_7: Keycode = Keycode::A;
pub const KEY_8: Keycode = Keycode::S;
pub const KEY_9: Keycode = Keycode::D;
pub const KEY_E: Keycode = Keycode::F;

pub const KEY_A: Keycode = Keycode::Z;
pub const KEY_0: Keycode = Keycode::X;
pub const KEY_B: Keycode = Keycode::C;
pub const KEY_F: Keycode = Keycode::V;

const WINDOW_WIDTH: u32 = (emulator::DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (emulator::DISPLAY_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    // SDL setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("chip8emu", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Emulator::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    'game_loop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game_loop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key_to_button(key) {
                        chip8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key_to_button(key) {
                        chip8.keypress(k, false);
                    }
                }
                _ => (),
            }
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas);
    }
}

fn draw_screen(emu: &Emulator, canvas: &mut Canvas<Window>) {
    // Clear canvas
    canvas.set_draw_color(Color::RGB(BG_RGB.0, BG_RGB.1, BG_RGB.2));
    canvas.clear();

    let screen_buf = emu.get_display();
    // Set to foreground colour, iterate thru pixels, check if should draw
    canvas.set_draw_color(Color::RGB(FG_RGB.0, FG_RGB.1, FG_RGB.2));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert index to 2D [x,y] position
            let x = (i % emulator::DISPLAY_WIDTH) as u32;
            let y = (i / emulator::DISPLAY_WIDTH) as u32;

            // Draw scaled-up rectangle @ [x,y]
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key_to_button(key: Keycode) -> Option<usize> {
    match key {
        KEY_1 => Some(0x1),
        KEY_2 => Some(0x2),
        KEY_3 => Some(0x3),
        KEY_C => Some(0xC),
        KEY_4 => Some(0x4),
        KEY_5 => Some(0x5),
        KEY_6 => Some(0x6),
        KEY_D => Some(0xD),
        KEY_7 => Some(0x7),
        KEY_8 => Some(0x8),
        KEY_9 => Some(0x9),
        KEY_E => Some(0xE),
        KEY_A => Some(0xA),
        KEY_0 => Some(0x0),
        KEY_B => Some(0xB),
        KEY_F => Some(0xF),
        _ => None,
    }
}
