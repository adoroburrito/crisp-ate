enum CrispsAteDecodedOpcodes {
    // TO-DO -> fix: 0NNN, 1NNN, 2NNN, ANNN, BNNN, DXYN
    // 12-bit max! (0-4095) 16-bit is too large (0-65535)
    Call(u16),                              // 0NNN
    ClearDisplay,                           // 00E0
    Return,                                 // 00EE
    Jump(u16),                              // 1NNN
    CallSubRoutine(u16),                    // 2NNN
    SkipIfVXEquals(u8),                     // 3XNN
    SkipIfVXNotEqual(u8),                   // 4XNN
    SkipIfVXEqualsVY,                       // 5XY0
    SetVX(u8),                              // 6XNN
    AddToVX(u8),                            // 7XNN
    SetVXToVY,                              // 8XY0
    SetVXToVXorVY,                          // 8XY1
    SetVXToVXandVY,                         // 8XY2
    SetVXToVXxorVY,                         // 8XY3
    AddVYtoVX,                              // 8XY4
    SubtractVYFromVX,                       // 8XY5
    StoreLeastBitOfVXAndShiftVXRight,       // 8XY6
    SetVXToVYMinusVX,                       // 8XY7
    StoreLeastBitOfVXAndShiftVXLeft,        // 8XYE
    SkipIfVXNotEqualVY,                     // 9XY0
    SetIAddress(u16),                       // ANNN
    JumpToAddress(u16),                     // BNNN
    SetVXToBitwiseANDWithSaltAndRandom(u8), // CXNN
    DrawSpriteAt(u16),                      // DXYN
    SkipIfKeyAtVXIsPressed,                 // EX9E
    SkipIfKeyAtVXIsNotPressed,              // EXA1
    SetVXToDelayValue,                      // FX07
    GetKeyToVX,                             // FX0A
    SetDelayToVX,                           // FX15
    SetSoundToVX,                           // FX18
    AddVXToI,                               // FX1E
    SetIToLocationOfVXChar,                 // FX29
    StoreBinaryCodedDecimalVX,              // FX33
    StoreFromV0ToVXStartingFromI,           // FX55
    FillFromV0ToVXStartingFromI,            // FX65
}

#[derive(Clone)]
struct CrispAteTimers {
    delay: u8,
    sound: u8,
}

impl CrispAteTimers {
    fn new() -> Self {
        CrispAteTimers { delay: 0, sound: 0 }
    }
}

#[derive(Clone)]
struct CrispAteRegisters {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    I: u16,
    PC: u16,
}

impl CrispAteRegisters {
    fn new() -> Self {
        CrispAteRegisters {
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            I: 0,
            PC: 0,
        }
    }
}
#[derive(Clone)]
struct CrispAte {
    memory: [u8; 4096],
    registers: CrispAteRegisters,
    screen: [bool; 64 * 32],
    timers: CrispAteTimers,
}

impl CrispAte {
    fn new() -> Self {
        let memory: [u8; 4096] = [0; 4096];
        let registers = CrispAteRegisters::new();
        let screen: [bool; 64 * 32] = [false; 64 * 32];
        let timers = CrispAteTimers::new();

        CrispAte {
            memory,
            registers,
            screen,
            timers,
        }
    }
}

fn main() {
    let vm = CrispAte::new();
}
