use std::fmt;

use crate::utils::hex;

#[derive(PartialEq, Eq)]
pub enum CrispsAteDecodedOpcodes {
    // TO-DO -> fix: 0NNN, 1NNN, 2NNN, ANNN, BNNN, DXYN
    // 12-bit max! (0-4095) 16-bit is too large (0-65535)
    Call(u16),                                    // 0NNN (NNN)
    ClearDisplay,                                 // 00E0
    Return,                                       // 00EE
    Jump(u16),                                    // 1NNN (NNN)
    CallSubRoutine(u16),                          // 2NNN (NNN)
    SkipIfVXEquals(u16, u16),                     // 3XNN (X, NN)
    SkipIfVXNotEqual(u16, u16),                   // 4XNN (X, NN)
    SkipIfVXEqualsVY(u16, u16),                   // 5XY0 (X, Y)
    SetVX(u16, u16),                              // 6XNN (X, NN)
    AddToVX(u16, u16),                            // 7XNN (X, NN)
    SetVXToVY(u16, u16),                          // 8XY0 (X, Y)
    SetVXToVXorVY(u16, u16),                      // 8XY1 (X, Y)
    SetVXToVXandVY(u16, u16),                     // 8XY2 (X, Y)
    SetVXToVXxorVY(u16, u16),                     // 8XY3 (X, Y)
    AddVYtoVX(u16, u16),                          // 8XY4 (X, Y)
    SubtractVYFromVX(u16, u16),                   // 8XY5 (X, Y)
    StoreLeastBitOfVXAndShiftVXRight(u16),        // 8XY6 (X)
    SetVXToVYMinusVX(u16, u16),                   // 8XY7 (X, Y)
    StoreMostBitOfVXAndShiftVXLeft(u16),          // 8XYE (X)
    SkipIfVXNotEqualVY(u16, u16),                 // 9XY0 (X, y)
    SetIAddress(u16),                             // ANNN (NNN)
    JumpToAddress(u16),                           // BNNN (NNN)
    SetVXToBitwiseANDWithSaltAndRandom(u16, u16), // CXNN (X, NN)
    DrawSpriteAt(u16, u16, u16),                  // DXYN (X, Y, N)
    SkipIfKeyAtVXIsPressed(u16),                  // EX9E (X)
    SkipIfKeyAtVXIsNotPressed(u16),               // EXA1 (X)
    SetVXToDelayValue(u16),                       // FX07 (X)
    GetKeyToVX(u16),                              // FX0A (X)
    SetDelayToVX(u16),                            // FX15 (X)
    SetSoundToVX(u16),                            // FX18 (X)
    AddVXToI(u16),                                // FX1E (X)
    SetIToLocationOfVXChar(u16),                  // FX29 (X)
    StoreBinaryCodedDecimalVX(u16),               // FX33 (X)
    StoreFromV0ToVXStartingFromI(u16),            // FX55 (X)
    FillFromV0ToVXStartingFromI(u16),             // FX65 (X)
    None(u16),                                    // Unknown
}

impl fmt::Debug for CrispsAteDecodedOpcodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = match self {
            CrispsAteDecodedOpcodes::Call(_) => "Call (0NNN)",
            CrispsAteDecodedOpcodes::ClearDisplay => "ClearDisplay (00E0)",
            CrispsAteDecodedOpcodes::Return => "Return (00EE)",
            CrispsAteDecodedOpcodes::Jump(_) => "Jump (1NNN)",
            CrispsAteDecodedOpcodes::CallSubRoutine(_) => "CallSubRoutine (2NNN)",
            CrispsAteDecodedOpcodes::SkipIfVXEquals(_, _) => "SkipIfVXEquals (3XNN)",
            CrispsAteDecodedOpcodes::SkipIfVXNotEqual(_, _) => "SkipIfVXNotEqual (4XNN)",
            CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(_, _) => "SkipIfVXEqualsVY (5XY0)",
            CrispsAteDecodedOpcodes::SetVX(_, _) => "SetVX (6XNN)",
            CrispsAteDecodedOpcodes::AddToVX(_, _) => "AddToVX (7XNN)",
            CrispsAteDecodedOpcodes::SetVXToVY(_, _) => "SetVXToVY (8XY0)",
            CrispsAteDecodedOpcodes::SetVXToVXorVY(_, _) => "SetVXToVXorVY (8XY1)",
            CrispsAteDecodedOpcodes::SetVXToVXandVY(_, _) => "SetVXToVXandVY (8XY2)",
            CrispsAteDecodedOpcodes::SetVXToVXxorVY(_, _) => "SetVXToVXxorVY (8XY3)",
            CrispsAteDecodedOpcodes::AddVYtoVX(_, _) => "AddVYtoVX (8XY4)",
            CrispsAteDecodedOpcodes::SubtractVYFromVX(_, _) => "SubtractVYFromVX (8XY5)",
            CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(_) => {
                "StoreLeastBitOfVXAndShiftVXRight (8XY6)"
            }
            CrispsAteDecodedOpcodes::SetVXToVYMinusVX(_, _) => "SetVXToVYMinusVX (8XY7)",
            CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(_) => {
                "StoreMostBitOfVXAndShiftVXLeft (8XYE)"
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(_, _) => "SkipIfVXNotEqualVY (9XY0)",
            CrispsAteDecodedOpcodes::SetIAddress(_) => "SetIAddress (ANNN)",
            CrispsAteDecodedOpcodes::JumpToAddress(_) => "JumpToAddress (BNNN)",
            CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(_, _) => {
                "SetVXToBitwiseANDWithSaltAndRandom (CXNN)"
            }
            CrispsAteDecodedOpcodes::DrawSpriteAt(_, _, _) => "DrawSpriteAt (DXYN)",
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(_) => "SkipIfKeyAtVXIsPressed (EX9E)",
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(_) => {
                "SkipIfKeyAtVXIsNotPressed (EXA1)"
            }
            CrispsAteDecodedOpcodes::SetVXToDelayValue(_) => "SetVXToDelayValue (FX07)",
            CrispsAteDecodedOpcodes::GetKeyToVX(_) => "GetKeyToVX (FX0A)",
            CrispsAteDecodedOpcodes::SetDelayToVX(_) => "SetDelayToVX (FX15)",
            CrispsAteDecodedOpcodes::SetSoundToVX(_) => "SetSoundToVX (FX18)",
            CrispsAteDecodedOpcodes::AddVXToI(_) => "AddVXToI (FX1E)",
            CrispsAteDecodedOpcodes::SetIToLocationOfVXChar(_) => "SetIToLocationOfVXChar (FX29)",
            CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(_) => {
                "StoreBinaryCodedDecimalVX (FX33)"
            }
            CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(_) => {
                "StoreFromV0ToVXStartingFromI (FX55)"
            }
            CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(_) => {
                "FillFromV0ToVXStartingFromI (FX65)"
            }
            CrispsAteDecodedOpcodes::None(_) => "None (Unknown)",
        };

        let values_hex: Vec<String> = match self {
            CrispsAteDecodedOpcodes::Call(val1) => vec![hex(*val1)],
            CrispsAteDecodedOpcodes::ClearDisplay => vec![],
            CrispsAteDecodedOpcodes::Return => vec![],
            CrispsAteDecodedOpcodes::Jump(val1) => vec![hex(*val1)],
            CrispsAteDecodedOpcodes::CallSubRoutine(val1) => vec![hex(*val1)],
            CrispsAteDecodedOpcodes::SkipIfVXEquals(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqual(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVX(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::AddToVX(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVXorVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVXandVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVXxorVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::AddVYtoVX(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SubtractVYFromVX(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(val1) => {
                vec![hex(*val1 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVYMinusVX(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(val1) => {
                vec![hex(*val1 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetIAddress(val1) => vec![hex(*val1)],
            CrispsAteDecodedOpcodes::JumpToAddress(val1) => vec![hex(*val1)],
            CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(val1, val2) => {
                vec![hex(*val1 as u16), hex(*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::DrawSpriteAt(val1, val2, val3) => {
                vec![hex(*val1 as u16), hex(*val2 as u16), hex(*val3 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetVXToDelayValue(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::GetKeyToVX(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetDelayToVX(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetSoundToVX(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::AddVXToI(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetIToLocationOfVXChar(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(val1) => vec![hex(*val1 as u16)],
            CrispsAteDecodedOpcodes::None(val1) => vec![hex(*val1)],
        };

        let values_dec: Vec<u16> = match self {
            CrispsAteDecodedOpcodes::Call(val1) => vec![*val1],
            CrispsAteDecodedOpcodes::ClearDisplay => vec![],
            CrispsAteDecodedOpcodes::Return => vec![],
            CrispsAteDecodedOpcodes::Jump(val1) => vec![*val1],
            CrispsAteDecodedOpcodes::CallSubRoutine(val1) => vec![*val1],
            CrispsAteDecodedOpcodes::SkipIfVXEquals(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfVXNotEqual(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfVXEqualsVY(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVX(val1, val2) => vec![(*val1 as u16), (*val2 as u16)],
            CrispsAteDecodedOpcodes::AddToVX(val1, val2) => vec![(*val1 as u16), (*val2 as u16)],
            CrispsAteDecodedOpcodes::SetVXToVY(val1, val2) => vec![(*val1 as u16), (*val2 as u16)],
            CrispsAteDecodedOpcodes::SetVXToVXorVY(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVXandVY(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetVXToVXxorVY(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::AddVYtoVX(val1, val2) => vec![(*val1 as u16), (*val2 as u16)],
            CrispsAteDecodedOpcodes::SubtractVYFromVX(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::StoreLeastBitOfVXAndShiftVXRight(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetVXToVYMinusVX(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::StoreMostBitOfVXAndShiftVXLeft(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SkipIfVXNotEqualVY(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::SetIAddress(val1) => vec![(*val1)],
            CrispsAteDecodedOpcodes::JumpToAddress(val1) => vec![(*val1)],
            CrispsAteDecodedOpcodes::SetVXToBitwiseANDWithSaltAndRandom(val1, val2) => {
                vec![(*val1 as u16), (*val2 as u16)]
            }
            CrispsAteDecodedOpcodes::DrawSpriteAt(val1, val2, val3) => {
                vec![(*val1 as u16), (*val2 as u16), (*val3 as u16)]
            }
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsPressed(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SkipIfKeyAtVXIsNotPressed(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetVXToDelayValue(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::GetKeyToVX(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetDelayToVX(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetSoundToVX(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::AddVXToI(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::SetIToLocationOfVXChar(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::StoreBinaryCodedDecimalVX(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::StoreFromV0ToVXStartingFromI(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::FillFromV0ToVXStartingFromI(val1) => vec![(*val1 as u16)],
            CrispsAteDecodedOpcodes::None(val1) => vec![(*val1)],
        };

        write!(f, "{} -> hex {:?} | dec {:?}", name, values_hex, values_dec)
    }
}

#[derive(Debug)]
pub struct CrispAteTimers {
    pub delay: u16,
    pub sound: u16,
}

impl CrispAteTimers {
    pub fn new() -> Self {
        CrispAteTimers { delay: 0, sound: 0 }
    }
}

#[derive(Debug)]
pub struct CrispAteRegisters {
    pub v_0: u16,
    pub v_1: u16,
    pub v_2: u16,
    pub v_3: u16,
    pub v_4: u16,
    pub v_5: u16,
    pub v_6: u16,
    pub v_7: u16,
    pub v_8: u16,
    pub v_9: u16,
    pub v_a: u16,
    pub v_b: u16,
    pub v_c: u16,
    pub v_d: u16,
    pub v_e: u16,
    pub v_f: u16,
    pub address: u16,
    pub program_counter: u16,
    pub draw_flag: bool,
}

impl CrispAteRegisters {
    pub fn new() -> Self {
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
            draw_flag: false,
        }
    }
}
