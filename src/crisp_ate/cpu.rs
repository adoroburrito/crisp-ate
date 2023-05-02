use super::registers::{CrispAteRegisters, CrispAteTimers, CrispsAteDecodedOpcodes};
use super::runtime::CrispAteRuntime;
use crate::utils::hex;

enum Digit {
    First(u16),
    Second(u16),
    Third(u16),
    Last(u16),
    LastTwo(u16),
    LastThree(u16),
}

fn get_digit(input: Digit) -> u16 {
    const FIRST_DIGIT: u16 = 0xF000;
    const SECOND_DIGIT: u16 = 0x0F00;
    const THIRD_DIGIT: u16 = 0x00F0;
    const LAST_DIGIT: u16 = 0x000F;
    const LAST_TWO_DIGITS: u16 = 0x00FF;
    const LAST_THREE_DIGITS: u16 = 0x0FFF;

    match input {
        Digit::First(opcode) => (opcode & FIRST_DIGIT) >> 12,
        Digit::Second(opcode) => (opcode & SECOND_DIGIT) >> 8,
        Digit::Third(opcode) => (opcode & THIRD_DIGIT) >> 4,
        Digit::Last(opcode) => opcode & LAST_DIGIT,
        Digit::LastTwo(opcode) => {
            let last_two_digits_hex = opcode & LAST_TWO_DIGITS;
            u16::from_str_radix(&format!("{:X}", last_two_digits_hex), 16).unwrap()
        }
        Digit::LastThree(opcode) => {
            let last_three_digits_hex = opcode & LAST_THREE_DIGITS;

            u16::from_str_radix(&format!("{:X}", last_three_digits_hex), 16).unwrap()
        }
    }
}

fn decode_opcode(opcode: u16) -> CrispsAteDecodedOpcodes {
    match get_digit(Digit::First(opcode)) {
        0x0 => match get_digit(Digit::Last(opcode)) {
            0x0 => CrispsAteDecodedOpcodes::ClearDisplay,
            0xE => CrispsAteDecodedOpcodes::Return,
            _ => CrispsAteDecodedOpcodes::Call(get_digit(Digit::LastThree(opcode))),
        },
        0xA => CrispsAteDecodedOpcodes::SetIAddress(get_digit(Digit::LastThree(opcode))),
        0xB => CrispsAteDecodedOpcodes::JumpToAddress(get_digit(Digit::LastThree(opcode))),
        0xC => CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0xD => CrispsAteDecodedOpcodes::DrawSpriteAt(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::Third(opcode)),
            get_digit(Digit::Last(opcode)),
        ),
        0xE => match get_digit(Digit::Last(opcode)) {
            0x1 => {
                CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(get_digit(Digit::Second(opcode)))
            }
            0xE => {
                CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(get_digit(Digit::Second(opcode)))
            }
            _ => CrispsAteDecodedOpcodes::None(opcode),
        },
        0xF => match get_digit(Digit::LastTwo(opcode)) {
            0x07 => CrispsAteDecodedOpcodes::SetVXToDelayValue(get_digit(Digit::Second(opcode))),
            0x0A => CrispsAteDecodedOpcodes::GetKeyToVX(get_digit(Digit::Second(opcode))),
            0x15 => CrispsAteDecodedOpcodes::SetDelayToVX(get_digit(Digit::Second(opcode))),
            0x18 => CrispsAteDecodedOpcodes::SetSoundToVX(get_digit(Digit::Second(opcode))),
            0x1E => CrispsAteDecodedOpcodes::AddVXToI(get_digit(Digit::Second(opcode))),
            0x29 => {
                CrispsAteDecodedOpcodes::SetIToLocationOfVXChar(get_digit(Digit::Second(opcode)))
            }
            0x33 => {
                CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(get_digit(Digit::Second(opcode)))
            }
            0x55 => CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(get_digit(
                Digit::Second(opcode),
            )),
            0x65 => CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(get_digit(Digit::Second(
                opcode,
            ))),
            _ => CrispsAteDecodedOpcodes::None(opcode),
        },
        0x1 => CrispsAteDecodedOpcodes::Jump(get_digit(Digit::LastThree(opcode))),
        0x2 => CrispsAteDecodedOpcodes::CallSubRoutine(get_digit(Digit::LastThree(opcode))),
        0x3 => CrispsAteDecodedOpcodes::SkipIfVXEquals(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0x4 => CrispsAteDecodedOpcodes::SkipIfVXNotEqual(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0x5 => CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0x6 => CrispsAteDecodedOpcodes::SetVX(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0x7 => CrispsAteDecodedOpcodes::AddToVX(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::LastTwo(opcode)),
        ),
        0x8 => match get_digit(Digit::Last(opcode)) {
            0x0 => CrispsAteDecodedOpcodes::SetVXToVY(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x1 => CrispsAteDecodedOpcodes::SetVXToVXorVY(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x2 => CrispsAteDecodedOpcodes::SetVXToVXandVY(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x3 => CrispsAteDecodedOpcodes::SetVXToVXxorVY(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x4 => CrispsAteDecodedOpcodes::AddVYtoVX(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x5 => CrispsAteDecodedOpcodes::SubtractVYFromVX(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0x6 => CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(get_digit(
                Digit::Second(opcode),
            )),
            0x7 => CrispsAteDecodedOpcodes::SetVXToVYMinusVX(
                get_digit(Digit::Second(opcode)),
                get_digit(Digit::Third(opcode)),
            ),
            0xE => CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(get_digit(
                Digit::Second(opcode),
            )),
            _ => CrispsAteDecodedOpcodes::None(opcode),
        },
        0x9 => CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(
            get_digit(Digit::Second(opcode)),
            get_digit(Digit::Third(opcode)),
        ),
        _ => CrispsAteDecodedOpcodes::None(opcode),
    }
}

#[derive(Debug)]
pub struct CrispAte {
    memory: [u8; 4096],
    pub registers: CrispAteRegisters,
    screen: [bool; 64 * 32],
    pub timers: CrispAteTimers,
    pub runtime: CrispAteRuntime,
}

impl CrispAte {
    pub fn new() -> Self {
        let memory: [u8; 4096] = [0; 4096];
        let registers = CrispAteRegisters::new();
        let screen: [bool; 64 * 32] = [false; 64 * 32];
        let timers = CrispAteTimers::new();
        let runtime = CrispAteRuntime::new();

        CrispAte {
            memory,
            registers,
            screen,
            timers,
            runtime,
        }
    }

    pub fn init(&mut self, file_bytes: [u8; 3584]) {
        // populate memory with font
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // load program in memory, starting in 0x200
        let mut fb_index = 0;
        for address in 0x200..=0xFFF {
            self.memory[address] = file_bytes[fb_index];

            fb_index += 1;
        }
        println!("Program loaded.");

        let mut index = 0x50;
        for byte in fontset {
            self.memory[index] = byte;

            index += 1;
        }
        println!("Fontset loaded.");

        // set program counter to start of the program
        self.registers.program_counter = 0x200;

        println!("Program counter set.");
    }

    fn fetch_and_decode(&self) -> CrispsAteDecodedOpcodes {
        let program_counter: usize = self.registers.program_counter.into();

        println!(
            "Fetching and decoding opcode at program counter: {}",
            program_counter
        );

        // gets byte at program counter
        let opcode_first_byte = self.memory[program_counter];
        let opcode_second_byte = self.memory[program_counter + 1];
        let result: u16 = (opcode_first_byte as u16) << 8 | opcode_second_byte as u16;

        let opcode = result & 0xFFFF;

        println!("Got opcode: {}", hex(opcode));

        decode_opcode(opcode)
    }

    fn find_v_register(&mut self, v_no: u16) -> &mut u16 {
        match v_no {
            0x0 => &mut self.registers.v_0,
            0x1 => &mut self.registers.v_1,
            0x2 => &mut self.registers.v_2,
            0x3 => &mut self.registers.v_3,
            0x4 => &mut self.registers.v_4,
            0x5 => &mut self.registers.v_5,
            0x6 => &mut self.registers.v_6,
            0x7 => &mut self.registers.v_7,
            0x8 => &mut self.registers.v_8,
            0x9 => &mut self.registers.v_9,
            0xA => &mut self.registers.v_a,
            0xB => &mut self.registers.v_b,
            0xC => &mut self.registers.v_c,
            0xD => &mut self.registers.v_d,
            0xE => &mut self.registers.v_e,
            0xF => &mut self.registers.v_f,
            _ => panic!("Unknown register!"),
        }
    }

    fn execute(&mut self, opcode: CrispsAteDecodedOpcodes) {
        println!("Trying to execute opcode: {:#04x?}", opcode);
        match opcode {
            CrispsAteDecodedOpcodes::None(opcode) => panic!("Unknown opcode: {:#04x?}", opcode),
            CrispsAteDecodedOpcodes::AddToVX(v_no, nibble) => {
                // 7XNN -> Adds NN to VX. (Carry flag is not changed);
                // v_no -> X
                // nibble -> NN
                *self.find_v_register(v_no) += nibble;
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::AddVXToI(v_no) => {
                // FX1E -> Adds VX to I. VF is not affected.
                // v_no -> X
                let vx = *self.find_v_register(v_no);

                self.registers.address += vx as u16;
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::AddVYtoVX(v_x_no, v_y_no) => {
                // 8XY4 -> Adds VY to VX.
                // VF is set to 1 when there's a carry, and to 0 when there is not.
                // v_x_no -> X
                // v_y_no -> Y
                if (*self.find_v_register(v_y_no) >> 4) > (0xFF - *self.find_v_register(v_y_no)) {
                    self.registers.v_f = 1;
                } else {
                    self.registers.v_f = 0;
                }

                *self.find_v_register(v_y_no) += *self.find_v_register(v_x_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::Call(nibble) => {
                // 0NNN -> Calls machine code routine (RCA 1802 for COSMAC VIP)
                // at address NNN. Not necessary for most ROMs.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::CallSubRoutine(nibble) => {
                // 2NNN -> Calls subroutine at NNN.
                // nibble -> NNN
                self.runtime.stack[self.runtime.stack_pointer] = self.registers.program_counter;
                self.runtime.stack_pointer += 1;
                self.registers.program_counter = nibble;
            }
            CrispsAteDecodedOpcodes::ClearDisplay => {
                // 00E0 -> Clears the screen.
                for (i, pixel) in self.screen.iter_mut().enumerate() {
                    *pixel = false;
                }
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::DrawSpriteAt(x, y, nibble) => {
                // DXYN -> Draws a sprite at coordinate (VX, VY) that has a width
                // of 8 pixels and a height of N pixels. Each row of 8 pixels is
                // read as bit-coded starting from memory location I; I value does
                // not change after the execution of this instruction. As described
                // above, VF is set to 1 if any screen pixels are flipped from set
                //to unset when the sprite is drawn, and to 0 if that does not happen
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(v_no) => {
                // FX65 -> Fills from V0 to VX (including VX) with values from memory,
                // starting at address I. The offset from I is increased by 1 for each value written,
                // but I itself is left unmodified.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::GetKeyToVX(v_no) => {
                // FX0A -> A key press is awaited, and then stored in VX.
                // (Blocking Operation. All instruction halted until next key event);
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::Jump(nibble) => {
                // 1NNN -> Jumps to address NNN
                // nibble -> NNN
                self.registers.program_counter = nibble;
            }
            CrispsAteDecodedOpcodes::JumpToAddress(nibble) => {
                // BNNN -> Jump to address NNN plus V0
                // nibble -> NNN
                let target = nibble + (self.registers.v_0 as u16);
                self.registers.program_counter = target;
            }
            CrispsAteDecodedOpcodes::Return => {
                // 00EE -> Returns from a subroutine.
                self.registers.program_counter = self.runtime.stack[self.runtime.stack_pointer];
                self.runtime.stack_pointer -= 1;
            }
            CrispsAteDecodedOpcodes::SetDelayToVX(v_no) => {
                // FX15 -> Sets the delay timer to VX.
                self.timers.delay = *self.find_v_register(v_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetIAddress(nibble) => {
                // ANNN -> Sets I to the address NNN.
                self.registers.address = nibble;
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetIToLocationOfVXChar(v_no) => {
                // FX29 -> Sets I to the location of the sprite for the character in VX.
                // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SetSoundToVX(v_no) => {
                // FX18 -> Sets the sound timer to VX.
                self.timers.sound = *self.find_v_register(v_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVX(v_no, nibble) => {
                // 6XNN -> Sets VX to NN
                // v_no -> X
                // nibble -> NN

                *self.find_v_register(v_no) += nibble;
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(v_no, nibble) => {
                // CXNN -> Sets VX to the result of a bitwise and operation on a random number
                // (Typically: 0 to 255) and NN.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SetVXToDelayValue(v_no) => {
                // FX07 -> Sets VX to the value of the delay timer.
                *self.find_v_register(v_no) = self.timers.delay;
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToVXandVY(v_x_no, v_y_no) => {
                // 8XY2 -> Sets VX to VX and VY. (Bitwise AND operation);
                *self.find_v_register(v_x_no) =
                    *self.find_v_register(v_x_no) & *self.find_v_register(v_y_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToVXorVY(v_x_no, v_y_no) => {
                // 8XY1 -> 	Sets VX to VX or VY. (Bitwise OR operation);
                *self.find_v_register(v_x_no) =
                    *self.find_v_register(v_x_no) | *self.find_v_register(v_y_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToVXxorVY(v_x_no, v_y_no) => {
                // 8XY3 -> Sets VX to VX xor VY.
                *self.find_v_register(v_x_no) =
                    *self.find_v_register(v_x_no) ^ *self.find_v_register(v_y_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToVY(v_x_no, v_y_no) => {
                // 8XY0 -> 	Sets VX to the value of VY.
                // v_x_no -> X
                // v_y_no -> Y

                *self.find_v_register(v_x_no) = *self.find_v_register(v_y_no);
                self.registers.program_counter += 2;
            }
            CrispsAteDecodedOpcodes::SetVXToVYMinusVX(v_x_no, v_y_no) => {
                // 8XY7 -> Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(v_no) => {
                // EXA1 -> Skips the next instruction if the key stored in VX is not pressed.
                // (Usually the next instruction is a jump to skip a code block);
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(v_no) => {
                // EX9E -> Skips the next instruction if the key stored in VX is pressed.
                // (Usually the next instruction is a jump to skip a code block);
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SkipIfVXEquals(v_no, nibble) => {
                // 3XNN -> Skips the next instruction if VX equals NN.
                // (Usually the next instruction is a jump to skip a code block);
                // v_no -> X
                // nibble -> NN

                let vx = *self.find_v_register(v_no);

                match vx == nibble {
                    false => {
                        self.registers.program_counter += 2;
                    }
                    true => {
                        self.registers.program_counter += 4;
                    }
                }
            }
            CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(v_x_no, v_y_no) => {
                // 5XY0 -> Skips the next instruction if VX equals VY.
                // (Usually the next instruction is a jump to skip a code block);
                // v_x_no -> X
                // v_y_no -> Y

                let vx = *self.find_v_register(v_x_no);
                let vy = *self.find_v_register(v_y_no);

                match vx == vy {
                    false => {
                        self.registers.program_counter += 2;
                    }
                    true => {
                        self.registers.program_counter += 4;
                    }
                }
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqual(v_no, nibble) => {
                // 4XNN -> Skips the next instruction if VX does not equal NN.
                // (Usually the next instruction is a jump to skip a code block);
                // v_no -> X
                // nibble -> NN

                let vx = *self.find_v_register(v_no);

                match vx == nibble {
                    false => {
                        self.registers.program_counter += 4;
                    }
                    true => {
                        self.registers.program_counter += 2;
                    }
                }
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(v_x_no, v_y_no) => {
                // 9XY0 -> Skips the next instruction if VX does not equal VY.
                // (Usually the next instruction is a jump to skip a code block);
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(v_no) => {
                // FX33 -> Stores the binary-coded decimal representation of VX,
                // with the most significant of three digits at the address in I,
                // the middle digit at I plus 1, and the least significant digit at I plus 2.
                // (In other words, take the decimal representation of VX,
                // place the hundreds digit in memory at location in I,
                // the tens digit at location I+1, and the ones digit at location I+2.);
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(v_no) => {
                // FX55 -> Stores from V0 to VX (including VX) in memory,
                // starting at address I. The offset from I is increased by 1 for each value written,
                // but I itself is left unmodified
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(v_no) => {
                // 8XY6 -> Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(v_no) => {
                // 8XYE -> Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                unimplemented!()
            }
            CrispsAteDecodedOpcodes::SubtractVYFromVX(v_x_no, v_y_no) => {
                // 8XY5 -> VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                unimplemented!()
            }
        }
    }

    pub fn emulation_cyle(&mut self) {
        println!("--------------------");
        println!();
        println!("Starting emulation cycle...");
        let opcode = self.fetch_and_decode();
        println!("Detected opcode: {:#?}", opcode);
        self.execute(opcode);

        if self.timers.delay > 0 {
            self.timers.delay -= 1;
        }

        if self.timers.sound > 0 {
            if self.timers.sound == 1 {
                println!("BEEP!");
            }
            self.timers.sound -= 1;
        }

        println!("Registers state after running opcode:");
        println!("{:#?}", self.runtime);
        println!();

        println!("Runtime state after running opcode:");
        println!("{:#?}", self.runtime);
        println!();

        println!("Timers state after running opcode:");
        println!("{:#?}", self.timers);
        println!();

        println!("Cicle finished.");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_OPCODE_1: u16 = 0x1234;
    const TEST_OPCODE_2: u16 = 0xABCD;
    const TEST_OPCODE_3: u16 = 0x1FA7;

    #[test]
    fn can_get_first_digit() {
        let first_digit = get_digit(Digit::First(TEST_OPCODE_1));
        assert_eq!(first_digit, 0x1 as u16);

        let first_digit_2 = get_digit(Digit::First(TEST_OPCODE_2));
        assert_eq!(first_digit_2, 0xA as u16);

        let first_digit_3 = get_digit(Digit::First(TEST_OPCODE_3));
        assert_eq!(first_digit_3, 0x1 as u16);
    }

    #[test]
    fn can_get_second_digit() {
        let second_digit = get_digit(Digit::Second(TEST_OPCODE_1));
        assert_eq!(second_digit, 0x2 as u16);

        let second_digit_2 = get_digit(Digit::Second(TEST_OPCODE_2));
        assert_eq!(second_digit_2, 0xB as u16);

        let second_digit_3 = get_digit(Digit::Second(TEST_OPCODE_3));
        assert_eq!(second_digit_3, 0xF as u16);
    }

    #[test]
    fn can_get_third_digit() {
        let third_digit = get_digit(Digit::Third(TEST_OPCODE_1));
        assert_eq!(third_digit, 0x3 as u16);

        let third_digit_2 = get_digit(Digit::Third(TEST_OPCODE_2));
        assert_eq!(third_digit_2, 0xC as u16);

        let third_digit_3 = get_digit(Digit::Third(TEST_OPCODE_3));
        assert_eq!(third_digit_3, 0xA as u16);
    }

    #[test]
    fn can_get_fourth_digit() {
        let fourth_digit = get_digit(Digit::Last(TEST_OPCODE_1));
        assert_eq!(fourth_digit, 0x4 as u16);

        let fourth_digit_2 = get_digit(Digit::Last(TEST_OPCODE_2));
        assert_eq!(fourth_digit_2, 0xD as u16);

        let fourth_digit_3 = get_digit(Digit::Last(TEST_OPCODE_3));
        assert_eq!(fourth_digit_3, 0x7 as u16);
    }

    #[test]
    fn can_get_last_two_digits() {
        let last_two_digits = get_digit(Digit::LastTwo(TEST_OPCODE_1));
        assert_eq!(last_two_digits, 0x34 as u16);

        let last_two_digits_2 = get_digit(Digit::LastTwo(TEST_OPCODE_2));
        assert_eq!(last_two_digits_2, 0xCD as u16);

        let last_two_digits_3 = get_digit(Digit::LastTwo(TEST_OPCODE_3));
        assert_eq!(last_two_digits_3, 0xA7 as u16);
    }

    #[test]
    fn can_get_last_three_digits() {
        let last_three_digits = get_digit(Digit::LastThree(TEST_OPCODE_1));
        assert_eq!(last_three_digits, 0x234 as u16);

        let last_three_digits_2 = get_digit(Digit::LastThree(TEST_OPCODE_2));
        assert_eq!(last_three_digits_2, 0xBCD as u16);

        let last_three_digits_3 = get_digit(Digit::LastThree(TEST_OPCODE_3));
        assert_eq!(last_three_digits_3, 0xFA7 as u16);
    }

    #[test]
    fn can_properly_get_call_opcode() {
        //Call(u16) -> 0NNN (NNN)
        let sample_opcode = 0x0123;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::Call(0x123));
    }

    #[test]
    fn can_properly_get_clear_display_opcode() {
        //ClearDisplay -> 00E0
        let sample_opcode = 0x00E0;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::ClearDisplay);
    }

    #[test]
    fn can_properly_get_return_opcode() {
        // Return -> 00EE
        let sample_opcode = 0x00EE;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::Return);
    }

    #[test]
    fn can_properly_get_jump_opcode() {
        // Jump(u16) -> 1NNN
        let sample_opcode = 0x1987;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::Jump(0x987));
    }

    #[test]
    fn can_properly_get_call_subroutine_opcode() {
        // CallSubRoutine(u16) -> 2NNN
        let sample_opcode = 0x2525;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::CallSubRoutine(0x525));
    }

    #[test]
    fn can_properly_get_skipifvxequals_opcode() {
        // SkipIfVXEquals(u16, u16) -> 3XNN (X, NN)
        let sample_opcode = 0x3921;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SkipIfVXEquals(0x9, 0x21));
    }

    #[test]
    fn can_properly_get_skipifvxnotequals_opcode() {
        // SkipIfVXNotEqual(u16, u16) -> 4XNN (X, NN)
        let sample_opcode = 0x4198;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SkipIfVXNotEqual(0x1, 0x98));
    }

    #[test]
    fn can_properly_get_skipifvxequalsvy_opcode() {
        // SkipIfVXEqualsVY(u16, u16) -> 5XY0 (X, Y)
        let sample_opcode = 0x5410;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(0x4, 0x1));
    }

    #[test]
    fn can_properly_get_setvx_opcode() {
        // SetVX(u16, u16) -> 6XNN (X, NN)
        let sample_opcode = 0x6287;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVX(0x2, 0x87));
    }

    #[test]
    fn can_properly_get_addtovx_opcode() {
        // AddToVX(u16, u16) -> 7XNN (X, NN)
        let sample_opcode = 0x7927;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::AddToVX(0x9, 0x27));
    }

    #[test]
    fn can_properly_get_setvxtovy_opcode() {
        // SetVXToVY(u16, u16) -> 8XY0 (X, Y)
        let sample_opcode = 0x8920;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVY(0x9, 0x2));
    }

    #[test]
    fn can_properly_get_setvxtovxorvy_opcode() {
        // SetVXToVXorVY(u16, u16) -> 8XY1 (X, Y)
        let sample_opcode = 0x8291;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVXorVY(0x2, 0x9));
    }

    #[test]
    fn can_properly_get_setvxtovxandvy_opcode() {
        // SetVXToVXandVY(u16, u16) -> 8XY2 (X, Y)
        let sample_opcode = 0x8742;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVXandVY(0x7, 0x4));
    }

    #[test]
    fn can_properly_get_setvxtovxxorvy_opcode() {
        // SetVXToVXxorVY(u16, u16) -> 8XY3 (X, Y)
        let sample_opcode = 0x8373;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVXxorVY(0x3, 0x7));
    }

    #[test]
    fn can_properly_get_addvytovx_opcode() {
        // AddVYtoVX(u16, u16) -> 8XY4 (X, Y)
        let sample_opcode = 0x8714;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVXandVY(0x7, 0x1));
    }

    #[test]
    fn can_properly_get_subtractvyfromvx_opcode() {
        // SubtractVYFromVX(u16, u16) -> 8XY5 (X, Y)
        let sample_opcode = 0x8915;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SubtractVYFromVX(0x9, 0x1));
    }

    #[test]
    fn can_properly_get_storeleastbitofvxandshiftvxright_opcode() {
        // StoreLeastBitOfVXAndShiftVXRight(u16) -> 8XY6 (X)
        let sample_opcode = 0x8276;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(0x2)
        );
    }

    #[test]
    fn can_properly_get_setvxtovyminusvx_opcode() {
        // SetVXToVYMinusVX(u16, u16) -> 8XY7 (X, Y)
        let sample_opcode = 0x8717;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToVYMinusVX(0x7, 0x1));
    }

    #[test]
    fn can_properly_get_storemostbitofvxandshiftvxleft_opcode() {
        // StoreMostBitOfVXAndShiftVXLeft(u16) -> 8XYE (X)
        let sample_opcode = 0x812E;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(0x1)
        );
    }

    #[test]
    fn can_properly_get_skipifvxnotequalvy_opcode() {
        // SkipIfVXNotEqualVY(u16, u16) -> 9XY0 (X, y)
        let sample_opcode = 0x9210;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(0x2, 0x1)
        );
    }

    #[test]
    fn can_properly_get_setiaddress_opcode() {
        // SetIAddress(u16) -> ANNN (NNN)
        let sample_opcode = 0xA987;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetIAddress(0x987));
    }

    #[test]
    fn can_properly_get_jumptoaddress_opcode() {
        // JumpToAddress(u16) -> BNNN (NNN)
        let sample_opcode = 0xB678;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::JumpToAddress(0x678));
    }

    #[test]
    fn can_properly_get_setvxtobitwiseandwithsaltandrandom_opcode() {
        // SetVXToBitwiseANDWithSaltAndRandom(u16, u16) -> CXNN (X, NN)
        let sample_opcode = 0xC208;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(0x2, 0x08)
        );
    }

    #[test]
    fn can_properly_get_drawspriteat_opcode() {
        // DrawSpriteAt(u16, u16, u16) -> DXYN (X, Y, N)
        let sample_opcode = 0xD135;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::DrawSpriteAt(0x1, 0x3, 0x5));
    }

    #[test]
    fn can_properly_get_skipifkeyatvxispressed_opcode() {
        // SkipIfKeyAtVXIsPressed(u16) -> EX9E (X)
        let sample_opcode = 0xE69E;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(0x6));
    }

    #[test]
    fn can_properly_get_skipifkeyatvxisnotpressed_opcode() {
        // SkipIfKeyAtVXIsNotPressed(u16) -> EXA1 (X)
        let sample_opcode = 0xE8A1;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(0x8)
        );
    }

    #[test]
    fn can_properly_get_setvxtodelayvalue_opcode() {
        // SetVXToDelayValue(u16) -> FX07 (X)
        let sample_opcode = 0xF107;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetVXToDelayValue(0x1));
    }

    #[test]
    fn can_properly_get_getkeytovx_opcode() {
        // GetKeyToVX(u16) -> FX0A (X)
        let sample_opcode = 0xF70A;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::GetKeyToVX(0x7));
    }

    #[test]
    fn can_properly_get_setdelaytovx_opcode() {
        // SetDelayToVX(u16) -> FX15 (X)
        let sample_opcode = 0xF415;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetDelayToVX(0x4));
    }

    #[test]
    fn can_properly_get_setsoundtovx_opcode() {
        // SetSoundToVX(u16) -> FX18 (X)
        let sample_opcode = 0xF018;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::SetSoundToVX(0x0));
    }

    #[test]
    fn can_properly_get_addvxtoi_opcode() {
        // AddVXToI(u16) -> FX1E (X)
        let sample_opcode = 0xF91E;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::AddVXToI(0x9));
    }

    #[test]
    fn can_properly_get_setitolocationofvxchar_opcode() {
        // SetIToLocationOfVXChar(u16) -> FX29 (X)
        let sample_opcode = 0xF329;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::AddVXToI(0x3));
    }

    #[test]
    fn can_properly_get_storebinarycodeddecimalvx_opcode() {
        // StoreBinaryCodedDecimalVX(u16) -> FX33 (X)
        let sample_opcode = 0xF133;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(0x1)
        );
    }

    #[test]
    fn can_properly_get_storefromv0tovxstartingfromi_opcode() {
        // StoreFromV0ToVXStartingFromI(u16) -> FX55 (X)
        let sample_opcode = 0xF955;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(0x9)
        );
    }

    #[test]
    fn can_properly_get_fillfromv0tovxstartingfromi_opcode() {
        // FillFromV0ToVXStartingFromI(u16) -> FX65 (X)
        let sample_opcode = 0xF365;
        let result = decode_opcode(sample_opcode);
        assert_eq!(
            result,
            CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(0x3)
        );
    }

    #[test]
    fn can_properly_get_none_opcode() {
        // None(u16) -> Unknown
        let sample_opcode = 0xE26A;
        let result = decode_opcode(sample_opcode);
        assert_eq!(result, CrispsAteDecodedOpcodes::None(sample_opcode));
    }
}
