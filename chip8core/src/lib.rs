//! Backend for `chip8emu`.
#![warn(missing_docs)]

pub mod emulator;
mod fontset;
pub mod opcodes;

// Re-exports
pub use emulator::Emulator;
