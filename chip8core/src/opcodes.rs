//! All the Chip-8 opcodes and their corresponding instruction implementations.
use rand::random;

use super::{
    emulator::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    Emulator,
};

/// Match the opcode to the corresponding instruction, then execute the instruction.
pub fn execute_opcode(emu: &mut Emulator, op: u16) {
    let digit1 = (op & 0xF000) >> 12;
    let digit2 = (op & 0x0F00) >> 8;
    let digit3 = (op & 0x00F0) >> 4;
    let digit4 = op & 0x000F;

    match (digit1, digit2, digit3, digit4) {
        // 0x0000 - Nop
        (0x0, 0x0, 0x0, 0x0) => nop(),
        // 0x00E0 - Clear display
        (0x0, 0x0, 0xE, 0x0) => cls(emu),
        // 0x00EE - Return from subroutine
        (0x0, 0x0, 0xE, 0xE) => ret(emu),
        // 0x1NNN - Jump to location NNN
        (0x1, _, _, _) => jp(emu, op & 0x0FFF),
        // 0x2NNN - Call subroutine at location NNN
        (0x2, _, _, _) => call(emu, op & 0x0FFF),
        // 0x3XKK - Skip next instruction iff VX == KK.
        (0x3, x, _, _) => se_vx_byte(emu, x, (op & 0x00FF) as u8),
        // 0x4XKK - Skip next instruction iff VX != KK.
        (0x4, x, _, _) => sne_vx_byte(emu, x, (op & 0x00FF) as u8),
        // 0x5XY0 - Skip next instrution iff VX == VY.
        (0x5, x, y, 0) => se_vx_vy(emu, x, y),
        // 0x6XKK - Set VX = KK.
        (0x6, x, _, _) => ld_vx_byte(emu, x, (op & 0x00FF) as u8),
        // 0x7XKK - Set VX = VX + KK.
        (0x7, x, _, _) => add_vx_byte(emu, x, (op & 0x00FF) as u8),
        // 0x8XY0 - Set VX = VY.
        (0x8, x, y, 0x0) => ld_vx_vy(emu, x, y),
        // 0x8XY1 - Set VX = OR VX, VY.
        (0x8, x, y, 0x1) => or(emu, x, y),
        // 0x8XY2 - Set VX = AND VX, VY.
        (0x8, x, y, 0x2) => and(emu, x, y),
        // 0x8XY3 - Set VX = XOR VX, VY.
        (0x8, x, y, 0x3) => xor(emu, x, y),
        // 0x8XY4 - Set VX = VX + VY.
        (0x8, x, y, 0x4) => add_vx_vy(emu, x, y),
        // 0x8XY5 - Set VX = VX - VY.
        (0x8, x, y, 0x5) => sub_vx_vy(emu, x, y),
        // 0x8XY6 - Set VX = VX SHR 1.
        (0x8, x, _, 0x6) => shr(emu, x),
        // 0x8XY7 - Set VX = VY - VX.
        (0x8, x, y, 0x7) => subn_vx_vy(emu, x, y),
        // 0x8XYE - Set VX = VX SHL 1.
        (0x8, x, _, 0xE) => shl(emu, x),
        // 0x9XY0 - Skip next instruction iff VX != VY.
        (0x9, x, y, 0x0) => sne_vx_vy(emu, x, y),
        // 0xANNN - Set I = NNN.
        (0xA, _, _, _) => ld_i_addr(emu, op & 0x0FFF),
        // 0xBNNN - Jump to location NNN + V0.
        (0xB, _, _, _) => jp_v0(emu, op & 0x0FFF),
        // 0xCXKK - Set VX = random byte AND KK.
        (0xC, x, _, _) => rnd(emu, x, (op & 0x00FF) as u8),
        // 0xDXYN - Display N-byte sprite @ [VX, VY] with VF = collision.
        (0xD, x, y, n) => drw(emu, x, y, n),
        // 0xEX9E - Skip next instruction iff key with value of VX is pressed.
        (0xE, x, 0x9, 0xE) => skp(emu, x),
        // 0xEXA1 - Skip next instruction iff key with value of VX is not pressed.
        (0xE, x, 0xA, 0x1) => sknp(emu, x),
        // 0xFX07 - Set VX = delay timer value.
        (0xF, x, 0x0, 0x7) => ld_vx_dt(emu, x),
        // 0xFX0A - Wait for key press, then store pressed key value in VX.
        (0xF, x, 0x0, 0xA) => ld_vx_k(emu, x),
        // 0xFX15 - Set delay timer = VX.
        (0xF, x, 0x1, 0x5) => ld_dt_vx(emu, x),
        // 0xFX18 - Set sound timer = VX.
        (0xF, x, 0x1, 0x8) => ld_st_vx(emu, x),
        // 0xFX1E - Set I += VX.
        (0xF, x, 0x1, 0xE) => add_i_vx(emu, x),
        // 0xFX29 - Set I = location of sprite for digit VX.
        (0xF, x, 0x2, 0x9) => ld_f_vx(emu, x),
        // 0xFX33 - Store BCD representation of VX at I.
        (0xF, x, 0x3, 0x3) => ld_b_vx(emu, x),
        // 0xFX55 - Store registers V0..=VX at I.
        (0xF, x, 0x5, 0x5) => ld_i_vx(emu, x),
        // 0xFX65 - Read registers V0..=VX from I.
        (0xF, x, 0x6, 0x5) => ld_vx_i(emu, x),
        // Unimplemented.
        // 0NNN - SYS addr is purposefully unimplemented. Typically ignored by modern interpreters
        // as it was only used on the old computers upon which Chip-8 was originally implemented.
        (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
    }
}

/// Do nothing.
fn nop() {}

/// Clear the display.
fn cls(emu: &mut Emulator) {
    emu.display = [false; DISPLAY_HEIGHT * DISPLAY_WIDTH];
}

/// Return from a subroutine.
fn ret(emu: &mut Emulator) {
    let return_addr = emu.pop();
    emu.program_counter = return_addr;
}

/// Jump to location `addr`.
fn jp(emu: &mut Emulator, addr: u16) {
    emu.program_counter = addr;
}

/// Call subroutine at `addr`.
fn call(emu: &mut Emulator, addr: u16) {
    emu.push(emu.program_counter);
    emu.program_counter = addr;
}

/// Skip next instruction iff Vx == `byte`.
fn se_vx_byte(emu: &mut Emulator, x: u16, byte: u8) {
    if emu.get_v(x) == byte {
        emu.program_counter += 2;
    }
}

/// Skip next instruction iff Vx == `byte`.
fn sne_vx_byte(emu: &mut Emulator, x: u16, byte: u8) {
    if emu.get_v(x) != byte {
        emu.program_counter += 2;
    }
}

/// Skip next instruction iff Vx == Vy.
fn se_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    if emu.get_v(x) == emu.get_v(y) {
        emu.program_counter += 2;
    }
}

/// Set Vx = `byte`.
fn ld_vx_byte(emu: &mut Emulator, x: u16, byte: u8) {
    emu.set_v(x, byte);
}

/// Set Vx = Vx + `byte`.
fn add_vx_byte(emu: &mut Emulator, x: u16, byte: u8) {
    emu.set_v(x, emu.get_v(x).wrapping_add(byte));
}

/// Set Vx = Vy.
fn ld_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    emu.set_v(x, emu.get_v(y));
}

/// Set Vx = bitwise Vx OR Vy.
fn or(emu: &mut Emulator, x: u16, y: u16) {
    emu.set_v(x, emu.get_v(x) | emu.get_v(y));
}

/// Set Vx = bitwise Vx AND Vy.
fn and(emu: &mut Emulator, x: u16, y: u16) {
    emu.set_v(x, emu.get_v(x) & emu.get_v(y));
}

/// Set Vx = bitwise Vx XOR Vy.
fn xor(emu: &mut Emulator, x: u16, y: u16) {
    emu.set_v(x, emu.get_v(x) ^ emu.get_v(y));
}

/// Set Vx = Vx + Vy; set VF = carry.
/// (VF = 1 if result > 255; else 0)
fn add_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    let (new_vx, carry) = emu.get_v(x).overflowing_add(emu.get_v(y));
    emu.set_v(x, new_vx);
    emu.set_v(0xF_usize, if carry { 1 } else { 0 });
}

/// Set Vx = Vx - Vy, set VF = NOT borrow.
/// (VF = 1 if Vx > Vy; else 0)
fn sub_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    let (new_vx, borrow) = emu.get_v(x).overflowing_sub(emu.get_v(y));
    emu.set_v(x, new_vx);
    emu.set_v(0xF_usize, if borrow { 0 } else { 1 });
}

/// Set Vx = Vx SHR 1.
/// (VF = least significant bit of Vx)
fn shr(emu: &mut Emulator, x: u16) {
    let vx = emu.get_v(x);
    let lsb = vx & 0x0001;
    emu.set_v(x, vx >> 1);
    emu.set_v(0xF_usize, lsb);
}

/// Set Vx = Vy - Vx, set VF = NOT borrow.
/// (VF = 1 if Vy > Vx; else 0)
fn subn_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    let (new_vx, borrow) = emu.get_v(y).overflowing_sub(emu.get_v(x));
    emu.set_v(x, new_vx);
    emu.set_v(0xF_usize, if borrow { 0 } else { 1 });
}

/// Set Vx = Vx SHL 1.
/// (VF = most significant bit of Vx)
fn shl(emu: &mut Emulator, x: u16) {
    let vx = emu.get_v(x);
    let msb = (vx >> 7) & 0x0001;
    emu.set_v(x, vx << 1);
    emu.set_v(0xF_usize, msb);
}

/// Skip next instruction iff Vx != Vy.
fn sne_vx_vy(emu: &mut Emulator, x: u16, y: u16) {
    if emu.get_v(x) != emu.get_v(y) {
        emu.program_counter += 2;
    }
}

/// Set I register = `addr`.
fn ld_i_addr(emu: &mut Emulator, addr: u16) {
    emu.i_register = addr;
}

/// Jump to location `addr` + V0.
fn jp_v0(emu: &mut Emulator, addr: u16) {
    emu.program_counter = addr + (emu.get_v(0_usize) as u16);
}

/// Set Vx = random byte AND `byte`.
fn rnd(emu: &mut Emulator, x: u16, byte: u8) {
    emu.set_v(x, random::<u8>() & byte);
}

/// Display `num_rows`-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/// (VF = 1 if XOR rendering of sprite causes pixels to be erased; else 0)
fn drw(emu: &mut Emulator, x: u16, y: u16, num_rows: u16) {
    // Keep track of whether any pixels were flipped.
    let mut pixels_flipped = false;

    // Starting coordinates
    let starting_col = emu.get_v(x);
    let starting_row = emu.get_v(y);

    // Iterate over sprite rows
    for row_offset in 0..num_rows {
        // Get pixel data for this row
        let row_pixels = emu.ram[(emu.i_register + row_offset) as usize];

        // Iterate over sprite cols
        for col_offset in 0..8 {
            // For each sprite pixel's location, if the pixel is already on, flip it.
            if (row_pixels & (0b1000_0000 >> col_offset)) != 0 {
                // Wrap sprite around screen.
                // TODO make sprite wrapping togglable.
                let x = (starting_col + col_offset) as usize % DISPLAY_WIDTH;
                let y = ((starting_row as u16) + row_offset) as usize % DISPLAY_HEIGHT;

                // Get pixel index for screen array.
                let idx = x + (DISPLAY_WIDTH * y);

                // Check if about to flip pixel, then set the pixel
                pixels_flipped |= emu.display[idx];
                emu.display[idx] ^= true;
            }
        }
    }

    // Set VF = collision.
    if pixels_flipped {
        emu.set_v(0xF_usize, 1);
    } else {
        emu.set_v(0xF_usize, 0);
    }
}

/// Skip next instruction if key with value of Vx is pressed.
fn skp(emu: &mut Emulator, x: u16) {
    if emu.keys[emu.get_v(x) as usize] {
        emu.program_counter += 2;
    }
}

/// Skip next instruction if key with value of Vx is not pressed.
fn sknp(emu: &mut Emulator, x: u16) {
    if !emu.keys[emu.get_v(x) as usize] {
        emu.program_counter += 2;
    }
}

/// Set Vx = current value of delay timer.
fn ld_vx_dt(emu: &mut Emulator, x: u16) {
    emu.set_v(x, emu.delay_timer);
}

/// Wait for key press (stopping all execution), then store value of presssed key in Vx.
fn ld_vx_k(emu: &mut Emulator, x: u16) {
    let mut is_key_pressed = false;

    for i in 0..emu.keys.len() {
        if emu.keys[i] {
            emu.set_v(x, i as u8);
            is_key_pressed = true;
            break;
        }
    }

    if !is_key_pressed {
        // Redo opcode
        emu.program_counter -= 2;
    }
}

/// Set delay timer = Vx.
fn ld_dt_vx(emu: &mut Emulator, x: u16) {
    emu.delay_timer = emu.get_v(x);
}

/// Set sound timer = Vx.
fn ld_st_vx(emu: &mut Emulator, x: u16) {
    emu.sound_timer = emu.get_v(x);
}

/// Set I register = I register + Vx.
fn add_i_vx(emu: &mut Emulator, x: u16) {
    emu.i_register = emu.i_register.wrapping_add(emu.get_v(x).into());
}

/// Set I = location of sprite for digit Vx.
fn ld_f_vx(emu: &mut Emulator, x: u16) {
    // Font is stored at the start of memory, so no memory location offset needed.
    // All sprites are 5 bytes.
    emu.i_register = (emu.get_v(x) as u16) * 5;
}

/// Store binary-coded decimal representation of Vx in memory locations I, I+1, I+2.
fn ld_b_vx(emu: &mut Emulator, x: u16) {
    // TODO use a better BCD algorithm
    let vx = emu.get_v(x) as f32;

    let hundreds = (vx / 100.0).floor() as u8;
    let tens = ((vx / 10.0) % 10.0).floor() as u8;
    let ones = (vx % 10.0) as u8;

    emu.ram[emu.i_register as usize] = hundreds;
    emu.ram[(emu.i_register + 1) as usize] = tens;
    emu.ram[(emu.i_register + 2) as usize] = ones;
}

/// Store registers V0-`x` in memory starting at location I.
fn ld_i_vx(emu: &mut Emulator, x: u16) {
    for i in 0..=x {
        emu.ram[(emu.i_register + i) as usize] = emu.get_v(i);
    }
}

/// Read registers V0-`x` from memory starting at location I.
fn ld_vx_i(emu: &mut Emulator, x: u16) {
    for i in 0..=x {
        emu.set_v(i, emu.ram[(emu.i_register + i) as usize]);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::emulator;

    use super::*;

    #[test]
    fn test_cls() {
        let mut emu = Emulator::default();

        // Set random pixel values
        for i in 0..emu.display.len() {
            emu.display[i] = random();
        }

        // Ensure at least one pixel is on
        emu.display[0] = true;
        assert!(emu.display[0]);

        // Clear screen
        execute_opcode(&mut emu, 0x00E0);

        // Ensure all pixels are off
        for pixel in emu.display {
            assert!(!pixel);
        }
    }

    #[test]
    fn test_jp() {
        let mut emu = Emulator::new();
        assert_eq!(emu.program_counter, emulator::START_ADDRESS);

        // Jump to location 0x5FE
        execute_opcode(&mut emu, 0x15FE);
        assert_eq!(emu.program_counter, 0x5FE);

        // Jump to location 0x89
        execute_opcode(&mut emu, 0x1089);
        assert_eq!(emu.program_counter, 0x89);
    }

    #[test]
    fn test_subroutines() {
        let mut emu = Emulator::default();
        assert_eq!(emu.program_counter, emulator::START_ADDRESS);
        assert_eq!(emu.stack_pointer, 0);

        // Add instruction: call subroutine at location 0xE06
        emu.write_instruction(emu.program_counter, 0x2E06_u16);

        // Add instruction: return from subroutine
        emu.write_instruction(0x0E06_usize, 0x00EE_u16);

        // Tick: should jump to location 0xE06
        emu.tick();
        assert_eq!(emu.stack_pointer, 1);
        assert_eq!(emu.program_counter, 0xE06);

        // Tick: should return from subroutine
        emu.tick();
        assert_eq!(emu.stack_pointer, 0);
        assert_eq!(emu.program_counter, emulator::START_ADDRESS + 0x2);
    }

    #[test]
    fn test_se_sne() {
        let mut emu = Emulator::default();
        emu.set_v(0_usize, 5);
        emu.set_v(1_usize, 5);
        emu.set_v(2_usize, 6);
        emu.set_v(0xD_usize, 0);
        emu.set_v(0xE_usize, 0);

        // Add instruction: skip next instruction iff V0 == 5.
        emu.write_instruction(emu.program_counter, 0x3005_u16);
        // Add instruction: set VD = 1. should be skipped!
        emu.write_instruction(emu.program_counter + 2, 0x6D01_u16);
        // Add instruction: set VE = 1. should be executed!
        emu.write_instruction(emu.program_counter + 4, 0x6E01_u16);

        // Skip next instruction iff V0 == 6.
        emu.write_instruction(emu.program_counter + 6, 0x3006_u16);
        // Set VD = 2. Should be executed!
        emu.write_instruction(emu.program_counter + 8, 0x6D02_u16);

        // Skip next instruction iff V0 != 5.
        emu.write_instruction(emu.program_counter + 10, 0x4005_u16);
        // Set VD = 3. Should be executed!
        emu.write_instruction(emu.program_counter + 12, 0x6D03_u16);

        // Skip next instruction iff V1 != 6.
        emu.write_instruction(emu.program_counter + 14, 0x4106_u16);
        // Set VE = 4. Should be skipped!
        emu.write_instruction(emu.program_counter + 16, 0x6E04_u16);

        // Skip next instruction iff V0 == V1.
        emu.write_instruction(emu.program_counter + 18, 0x5010_u16);
        // Set VE = 5. Should be skipped!
        emu.write_instruction(emu.program_counter + 20, 0x6E05_u16);

        // Skip next instruction iff V1 == V2.
        emu.write_instruction(emu.program_counter + 22, 0x5120_u16);
        // Set VD = 6. Should be executed!
        emu.write_instruction(emu.program_counter + 24, 0x6D06_u16);

        // Skip next instruction iff V0 != V1.
        emu.write_instruction(emu.program_counter + 26, 0x9010_u16);
        // Set VE = 7. Should be executed!
        emu.write_instruction(emu.program_counter + 28, 0x6E07_u16);

        // Skip next instruction iff V1 != V2.
        emu.write_instruction(emu.program_counter + 30, 0x9120_u16);
        // Set VD = 8. Should be skipped!
        emu.write_instruction(emu.program_counter + 32, 0x6D08_u16);
        // Noop
        emu.write_instruction(emu.program_counter + 34, 0x0000_u16);

        // Execute and test
        assert_eq!(emu.get_v(0xD_usize), 0);
        assert_eq!(emu.get_v(0xE_usize), 0);
        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 0);
        assert_eq!(emu.get_v(0xE_usize), 1);

        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 2);
        assert_eq!(emu.get_v(0xE_usize), 1);

        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 3);
        assert_eq!(emu.get_v(0xE_usize), 1);

        emu.tick();

        emu.tick();

        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 6);
        assert_eq!(emu.get_v(0xE_usize), 1);

        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 6);
        assert_eq!(emu.get_v(0xE_usize), 7);

        emu.tick();
        emu.tick();
        assert_eq!(emu.get_v(0xD_usize), 6);
        assert_eq!(emu.get_v(0xE_usize), 7);

        assert_eq!(emu.program_counter, emulator::START_ADDRESS + 36);
    }

    #[test]
    fn test_add_vx_byte() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 6);
        execute_opcode(&mut emu, 0x7001);
        assert_eq!(emu.get_v(0_usize), 7);
        execute_opcode(&mut emu, 0x7008);
        assert_eq!(emu.get_v(0_usize), 0xF);
        execute_opcode(&mut emu, 0x70F0);
        assert_eq!(emu.get_v(0_usize), 0xFF);
        // Test wrapping
        execute_opcode(&mut emu, 0x7005);
        assert_eq!(emu.get_v(0_usize), 4);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut emu = Emulator::default();

        emu.set_v(0_usize, 0xF);
        emu.set_v(1_usize, 0x7E);

        execute_opcode(&mut emu, 0x8010);
        assert_eq!(emu.get_v(0_usize), 0x7E);
        assert_eq!(emu.get_v(1_usize), 0x7E);
    }

    #[test]
    fn test_or() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0b_0101_1110);
        emu.set_v(1_usize, 0b_0010_0010);

        execute_opcode(&mut emu, 0x8011);
        assert_eq!(emu.get_v(0_usize), 0b_0111_1110);
        assert_eq!(emu.get_v(1_usize), 0b_0010_0010);
    }

    #[test]
    fn test_and() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0b_0101_1110);
        emu.set_v(1_usize, 0b_0110_0011);

        execute_opcode(&mut emu, 0x8012);
        assert_eq!(emu.get_v(0_usize), 0b_0100_0010);
        assert_eq!(emu.get_v(1_usize), 0b_0110_0011);
    }

    #[test]
    fn test_xor() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0b_0101_1110);
        emu.set_v(1_usize, 0b_0110_0011);

        execute_opcode(&mut emu, 0x8013);
        assert_eq!(emu.get_v(0_usize), 0b_0011_1101);
        assert_eq!(emu.get_v(1_usize), 0b_0110_0011);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0xB7);
        emu.set_v(1_usize, 0x1F);

        execute_opcode(&mut emu, 0x8104);
        assert_eq!(emu.get_v(0_usize), 0xB7);
        assert_eq!(emu.get_v(1_usize), 0xD6);
        assert_eq!(emu.get_v(0xF_usize), 0x0);

        // Test overflow
        emu.set_v(2_usize, 0xF0);
        emu.set_v(3_usize, 0x9D);

        execute_opcode(&mut emu, 0x8234);
        assert_eq!(emu.get_v(2_usize), 0x8D);
        assert_eq!(emu.get_v(3_usize), 0x9D);
        assert_eq!(emu.get_v(0xF_usize), 0x1);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0xB7);
        emu.set_v(1_usize, 0x1F);

        execute_opcode(&mut emu, 0x8015);
        assert_eq!(emu.get_v(0_usize), 0x98);
        assert_eq!(emu.get_v(1_usize), 0x1F);
        assert_eq!(emu.get_v(0xF_usize), 0x1);

        // Test borrow
        emu.set_v(2_usize, 0xA0);
        emu.set_v(3_usize, 0xB5);

        execute_opcode(&mut emu, 0x8235);
        assert_eq!(emu.get_v(2_usize), 0xEB);
        assert_eq!(emu.get_v(3_usize), 0xB5);
        assert_eq!(emu.get_v(0xF_usize), 0x0);
    }

    #[test]
    fn test_shrl() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0b_1010_1010);
        emu.set_v(1_usize, 0b_0101_0101);
        emu.set_v(2_usize, 0b_1010_1010);
        emu.set_v(3_usize, 0b_0101_0101);

        // shr no remainder
        execute_opcode(&mut emu, 0x80A6);
        assert_eq!(emu.get_v(0x0_usize), 0b_0101_0101);
        assert_eq!(emu.get_v(0xF_usize), 0x0);

        // shr remainder
        execute_opcode(&mut emu, 0x81B6);
        assert_eq!(emu.get_v(0x1_usize), 0b_0010_1010);
        assert_eq!(emu.get_v(0xF_usize), 0x1);

        // shl not too big
        execute_opcode(&mut emu, 0x830E);
        assert_eq!(emu.get_v(0x3_usize), 0b_1010_1010);
        assert_eq!(emu.get_v(0xF_usize), 0x0);

        // shl too big
        execute_opcode(&mut emu, 0x82FE);
        assert_eq!(emu.get_v(0x2_usize), 0b_0101_0100);
        assert_eq!(emu.get_v(0xF_usize), 0x1);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut emu = Emulator::new();

        emu.set_v(0_usize, 0xB7);
        emu.set_v(1_usize, 0x1F);

        execute_opcode(&mut emu, 0x8107);
        assert_eq!(emu.get_v(0_usize), 0xB7);
        assert_eq!(emu.get_v(1_usize), 0x98);
        assert_eq!(emu.get_v(0xF_usize), 0x1);

        // Test borrow
        emu.set_v(2_usize, 0xA0);
        emu.set_v(3_usize, 0xB5);

        execute_opcode(&mut emu, 0x8327);
        assert_eq!(emu.get_v(2_usize), 0xA0);
        assert_eq!(emu.get_v(3_usize), 0xEB);
        assert_eq!(emu.get_v(0xF_usize), 0x0);
    }

    #[test]
    fn test_ld_i_addr() {
        let mut emu = Emulator::new();
        emu.i_register = 0x000;
        execute_opcode(&mut emu, 0xA123);
        assert_eq!(emu.i_register, 0x123);
        execute_opcode(&mut emu, 0xAFFF);
        assert_eq!(emu.i_register, 0xFFF);
    }

    #[test]
    fn test_jp_v0() {
        let mut emu = Emulator::new();
        emu.set_v(0_usize, 0x12);
        execute_opcode(&mut emu, 0xBF00);
        assert_eq!(emu.program_counter, 0xF12);
    }
}
