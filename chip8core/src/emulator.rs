//! Emulator struct defining CPU functionality.
use std::default::Default;

use super::fontset::{FONTSET, FONTSET_SIZE};

// 64x32 monochrome display.
/// Display width.
pub const DISPLAY_WIDTH: usize = 64;
/// Display height.
pub const DISPLAY_HEIGHT: usize = 32;

// 4K RAM
const RAM_SIZE: usize = 4096;
// V registers. 16 8-bit registers; V0-VF.
const NUM_REGISTERS: usize = 16;
// Stack
const STACK_SIZE: usize = 16;
// 16-key hex keypad.
// Original Layout:
// 1 2 3 C
// 4 5 6 D
// 7 8 9 E
// A 0 B F
const NUM_KEYS: usize = 16;

// First 0x200 bytes reserved. Start at RAM address 0x200.
const START_ADDRESS: u16 = 0x200;

/// Emulator. Defines CPU functionality.
pub struct Emulator {
    /// Special register. Incremented by programs as they run.
    pub program_counter: u16,
    /// Random-access memory. The entire program is copied into RAM.
    pub ram: [u8; RAM_SIZE],
    /// Screen pixels. Monochrome; 1 bit per pixel.
    pub display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// V registers. 8 bits.
    pub v_registers: [u8; NUM_REGISTERS],
    /// I register. 16 bits. Used for indexing into RAM for reads/writes.
    pub i_register: u16,
    /// Stack pointer to locate the top of the stack.
    pub stack_pointer: u16,
    /// Stack.
    pub stack: [u16; STACK_SIZE],
    /// Keypad.
    pub keys: [bool; NUM_KEYS],
    /// Delay timer. Decrement every clock cycle, perform action when 0.
    pub delay_timer: u8,
    /// Sound timer. Decrement every clock cycle, emit noise when 0.
    pub sound_timer: u8,
}
impl Emulator {
    /// Create new emulator with default values.
    pub fn new() -> Self {
        let mut new_emu = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_registers: [0; NUM_REGISTERS],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };

        // Copy fontset into reserved section
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    /// Reset emulator to default values.
    pub fn reset(&mut self) {
        self.program_counter = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.v_registers = [0; NUM_REGISTERS];
        self.i_register = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    /// Push to stack.
    pub fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    /// Pop from stack.
    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    /// Basic CPU loop:
    /// - Fetch value from program at memory address defined by program counter.
    /// - Decode instruction.
    /// - Execute instruction. May modify CPU registers or RAM.
    pub fn tick(&mut self) {
        // I. Fetch
        let op = self.fetch();
        // II. Decode & III. Execute
    }

    /// Fetch opcode. All Chip-8 opcodes are exactly 2 bytes.
    fn fetch(&mut self) -> u16 {
        // Get the two bytes
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[self.program_counter as usize] as u16;
        // Combine together as Big Endian.
        let op = (higher_byte << 8) | lower_byte;
        // Increment program counter.
        self.program_counter += 2;
        op
    }

    /// Tick timers.
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO make noise
            }
            self.sound_timer -= 1;
        }
    }

    /// Convenience function: get the V register value at the given index.
    pub fn get_v<T: Into<usize>>(&self, index: T) -> u8 {
        self.v_registers[index.into()]
    }

    /// Convenience function: set the V register value at the given index.
    pub fn set_v<T, U>(&mut self, index: T, value: U)
    where
        T: Into<usize>,
        U: Into<u8>,
    {
        self.v_registers[index.into()] = value.into();
    }
}
impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
