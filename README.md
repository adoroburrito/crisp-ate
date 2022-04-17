# crisp-ate
A chip-8 emulator in Rust, using raylib for graphics. For emulation learning purposes.

Based on:
https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#1nnn-jump

# Usage

```bash
$ git clone https://github.com/adoroburrito/crisp-ate
$ cd crip-ate
$ cargo run "<path to chip-8 rom>"
```

# TO-DO
- [X] basics
  - [X] memory (8-bit array with 4096 positions)
  - [X] registers
  - [X] opcode enums
  - [X] load program into memory
- [ ] emulation
  - [X] fetch and decode opcode at program counter
  - [ ] execution
    - [X] simple opcodes (not much bitwise operations needed)
    - [ ] complex opcodes (some advanced bitwise operations needed)
  - [ ] draw frame (with raylib)
  - [ ] play sound timer beep (with raylib)
- [ ] completeness
  - [ ] test system with a test rom
  - [ ] test system with games
