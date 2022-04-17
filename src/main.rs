use std::env;
use std::io::ErrorKind;

#[derive(Debug)]
struct CrispAteRuntime {
    stack_pointer: usize,
    stack: [u16; 16],
}

impl CrispAteRuntime {
    fn new() -> Self {
        CrispAteRuntime {
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
#[derive(Debug)]
enum CrispsAteDecodedOpcodes {
    // TO-DO -> fix: 0NNN, 1NNN, 2NNN, ANNN, BNNN, DXYN
    // 12-bit max! (0-4095) 16-bit is too large (0-65535)
    Call(u16),                                  // 0NNN (NNN)
    ClearDisplay,                               // 00E0
    Return,                                     // 00EE
    Jump(u16),                                  // 1NNN (NNN)
    CallSubRoutine(u16),                        // 2NNN (NNN)
    SkipIfVXEquals(u8, u8),                     // 3XNN (X, NN)
    SkipIfVXNotEqual(u8, u8),                   // 4XNN (X, NN)
    SkipIfVXEqualsVY(u8, u8),                   // 5XY0 (X, Y)
    SetVX(u8, u8),                              // 6XNN (X, NN)
    AddToVX(u8, u8),                            // 7XNN (X, NN)
    SetVXToVY(u8, u8),                          // 8XY0 (X, Y)
    SetVXToVXorVY(u8, u8),                      // 8XY1 (X, Y)
    SetVXToVXandVY(u8, u8),                     // 8XY2 (X, Y)
    SetVXToVXxorVY(u8, u8),                     // 8XY3 (X, Y)
    AddVYtoVX(u8, u8),                          // 8XY4 (X, Y)
    SubtractVYFromVX(u8, u8),                   // 8XY5 (X, Y)
    StoreLeastBitOfVXAndShiftVXRight(u8),       // 8XY6 (X)
    SetVXToVYMinusVX(u8, u8),                   // 8XY7 (X, Y)
    StoreMostBitOfVXAndShiftVXLeft(u8),         // 8XYE (X)
    SkipIfVXNotEqualVY(u8, u8),                 // 9XY0 (X, y)
    SetIAddress(u16),                           // ANNN (NNN)
    JumpToAddress(u16),                         // BNNN (NNN)
    SetVXToBitwiseANDWithSaltAndRandom(u8, u8), // CXNN (X, NN)
    DrawSpriteAt(u8, u8, u8),                   // DXYN (X, Y, N)
    SkipIfKeyAtVXIsPressed(u8),                 // EX9E (X)
    SkipIfKeyAtVXIsNotPressed(u8),              // EXA1 (X)
    SetVXToDelayValue(u8),                      // FX07 (X)
    GetKeyToVX(u8),                             // FX0A (X)
    SetDelayToVX(u8),                           // FX15 (X)
    SetSoundToVX(u8),                           // FX18 (X)
    AddVXToI(u8),                               // FX1E (X)
    SetIToLocationOfVXChar(u8),                 // FX29 (X)
    StoreBinaryCodedDecimalVX(u8),              // FX33 (X)
    StoreFromV0ToVXStartingFromI(u8),           // FX55 (X)
    FillFromV0ToVXStartingFromI(u8),            // FX65 (X)
    None(u16),                                  // Unknown
}

#[derive(Debug)]
struct CrispAteTimers {
    delay: u8,
    sound: u8,
}

impl CrispAteTimers {
    fn new() -> Self {
        CrispAteTimers { delay: 0, sound: 0 }
    }
}

#[derive(Debug)]
struct CrispAteRegisters {
    v_0: u8,
    v_1: u8,
    v_2: u8,
    v_3: u8,
    v_4: u8,
    v_5: u8,
    v_6: u8,
    v_7: u8,
    v_8: u8,
    v_9: u8,
    v_a: u8,
    v_b: u8,
    v_c: u8,
    v_d: u8,
    v_e: u8,
    v_f: u8,
    address: u16,
    program_counter: u16,
}

impl CrispAteRegisters {
    fn new() -> Self {
        CrispAteRegisters {
            v_0: 0,
            v_1: 0,
            v_2: 0,
            v_3: 0,
            v_4: 0,
            v_5: 0,
            v_6: 0,
            v_7: 0,
            v_8: 0,
            v_9: 0,
            v_a: 0,
            v_b: 0,
            v_c: 0,
            v_d: 0,
            v_e: 0,
            v_f: 0,
            address: 0,
            program_counter: 0,
        }
    }
}
#[derive(Debug)]
struct CrispAte {
    memory: [u8; 4096],
    registers: CrispAteRegisters,
    screen: [bool; 64 * 32],
    timers: CrispAteTimers,
    runtime: CrispAteRuntime,
}

impl CrispAte {
    fn new() -> Self {
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

    fn init(&mut self, file_bytes: [u8; 3584]) {
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

        let mut index = 0x50;
        for byte in fontset {
            self.memory[index] = byte;

            index += 1;
        }

        // set program counter to start of the program
        self.registers.program_counter = 0x200;
    }

    fn fetch_and_decode(&self) -> CrispsAteDecodedOpcodes {
        let program_counter: usize = self.registers.program_counter.into();
        // gets byte at program counter
        let opcode_first_byte = self.memory[program_counter];
        let opcode_second_byte = self.memory[program_counter + 1];
        let result: u16 = (opcode_first_byte as u16) << 8 | opcode_second_byte as u16;

        let opcode = result & 0xFFFF;

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

    fn emulation_cyle(&mut self) {
        let opcode = self.fetch_and_decode();
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
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        println!("Usage: crisp-ate <fileName>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut available_memory: [u8; 3584] = [0; 3584];
    let program_bytes = match std::fs::read(filename) {
        Ok(bytes) => bytes,
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Not enough permissions to open the file!");
                std::process::exit(1);
            }
            ErrorKind::NotFound => {
                eprintln!("File not found.");
                std::process::exit(1);
            }
            _ => {
                eprintln!("Unable to open file!");
                std::process::exit(1);
            }
        },
    };

    if program_bytes.len() > available_memory.len() {
        eprintln!("File is too big for emulator!");
        std::process::exit(1);
    }

    for (i, byte) in program_bytes.iter().enumerate() {
        available_memory[i] = byte.to_owned()
    }

    let mut vm = CrispAte::new();

    println!("Initializing VM...");
    vm.init(available_memory);
    println!("VM initialized!");
    println!("{:#?}", vm);

    loop {
        vm.emulation_cyle();
    }
}
