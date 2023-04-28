use crate::utils::hex;
use super::runtime::CrispAteRuntime;
use super::registers::{ CrispAteRegisters, CrispAteTimers, CrispsAteDecodedOpcodes };

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

        println!("Fetching and decoding opcode at program counter: {}", program_counter);

        // gets byte at program counter
        let opcode_first_byte = self.memory[program_counter];
        let opcode_second_byte = self.memory[program_counter + 1];
        let result: u16 = (opcode_first_byte as u16) << 8 | opcode_second_byte as u16;

        let opcode = result & 0xFFFF;

        println!("Got opcode: {}", hex(opcode));

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => CrispsAteDecodedOpcodes::ClearDisplay,
                0x000E => CrispsAteDecodedOpcodes::Return,
                _ => CrispsAteDecodedOpcodes::Call(opcode & 0x0FFF),
            },
            0xA000 => CrispsAteDecodedOpcodes::SetIAddress(opcode & 0x0FFF),
            0xB000 => CrispsAteDecodedOpcodes::JumpToAddress(opcode & 0x0FFF),
            0xC000 => CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00FF) as u8,
            ),
            0xD000 => CrispsAteDecodedOpcodes::DrawSpriteAt(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00F0) as u8,
                (opcode & 0x000F) as u8,
            ),
            0xE000 => match opcode & 0x000F {
                0x0001 => {
                    CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed((opcode & 0x0F00) as u8)
                }
                0x000E => CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed((opcode & 0x0F00) as u8),
                _ => CrispsAteDecodedOpcodes::None(opcode),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => CrispsAteDecodedOpcodes::SetVXToDelayValue((opcode & 0x0F00) as u8),
                0x000A => CrispsAteDecodedOpcodes::GetKeyToVX((opcode & 0x0F00) as u8),
                0x0015 => CrispsAteDecodedOpcodes::SetDelayToVX((opcode & 0x0F00) as u8),
                0x0018 => CrispsAteDecodedOpcodes::SetSoundToVX((opcode & 0x0F00) as u8),
                0x001E => CrispsAteDecodedOpcodes::AddVXToI((opcode & 0x0F00) as u8),
                0x0029 => CrispsAteDecodedOpcodes::SetIToLocationOfVXChar((opcode & 0x0F00) as u8),
                0x0033 => {
                    CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX((opcode & 0x0F00) as u8)
                }
                0x0055 => {
                    CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI((opcode & 0x0F00) as u8)
                }
                0x0065 => {
                    CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI((opcode & 0x0F00) as u8)
                }
                _ => CrispsAteDecodedOpcodes::None(opcode),
            },
            0x1000 => CrispsAteDecodedOpcodes::Jump(opcode & 0x0FFF),
            0x2000 => CrispsAteDecodedOpcodes::CallSubRoutine(opcode & 0x0FFF),
            0x3000 => CrispsAteDecodedOpcodes::SkipIfVXEquals(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00FF) as u8,
            ),
            0x4000 => CrispsAteDecodedOpcodes::SkipIfVXNotEqual(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00FF) as u8,
            ),
            0x5000 => CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00FF) as u8,
            ),
            0x6000 => {
                CrispsAteDecodedOpcodes::SetVX((opcode & 0x0F00) as u8, (opcode & 0x00FF) as u8)
            }
            0x7000 => {
                CrispsAteDecodedOpcodes::AddToVX((opcode & 0x0F00) as u8, (opcode & 0x00FF) as u8)
            }
            0x8000 => match opcode & 0x000F {
                0x0000 => CrispsAteDecodedOpcodes::SetVXToVY(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0001 => CrispsAteDecodedOpcodes::SetVXToVXorVY(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0002 => CrispsAteDecodedOpcodes::SetVXToVXandVY(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0003 => CrispsAteDecodedOpcodes::SetVXToVXxorVY(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0004 => CrispsAteDecodedOpcodes::AddVYtoVX(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0005 => CrispsAteDecodedOpcodes::SubtractVYFromVX(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x0006 => CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(
                    (opcode & 0x0F00) as u8,
                ),
                0x0007 => CrispsAteDecodedOpcodes::SetVXToVYMinusVX(
                    (opcode & 0x0F00) as u8,
                    (opcode & 0x00F0) as u8,
                ),
                0x000E => {
                    CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft((opcode & 0x0F00) as u8)
                }
                _ => CrispsAteDecodedOpcodes::None(opcode),
            },
            0x9000 => CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(
                (opcode & 0x0F00) as u8,
                (opcode & 0x00F0) as u8,
            ),
            _ => CrispsAteDecodedOpcodes::None(opcode),
        }
    }

    fn find_v_register(&mut self, v_no: u8) -> &mut u8 {
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
