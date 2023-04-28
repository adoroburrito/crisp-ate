#[derive(Debug)]
pub struct CrispAteRuntime {
    pub stack_pointer: usize,
    pub stack: [u16; 16],
}

impl CrispAteRuntime {
    pub fn new() -> Self {
        CrispAteRuntime {
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
