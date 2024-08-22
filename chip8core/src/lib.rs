// 4K RAM
const RAM_SIZE: usize = 4096;

pub struct Emulator {
    // Special register. Incremented by programs as they run.
    program_counter: u16,
    // Random-access memory.
    ram: [u8; RAM_SIZE],
}
