mod crisp_ate;
use std::process;
use std::io;
mod utils;
use crisp_ate::cpu::CrispAte;
use crisp_ate::display::create_display;
use std::env;
use std::io::ErrorKind;
use dialog::DialogBox;

use crate::crisp_ate::display::draw_frame;

const MAX_PROGRAM_SIZE: usize = 3584;

fn get_program_bytes(filename: &str) -> Option<Vec<u8>> {
    match std::fs::read(filename) {
        Ok(bytes) => Some(bytes),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Not enough permissions to open the file!");
                None
            }
            ErrorKind::NotFound => {
                eprintln!("File not found.");
                None
            }
            _ => {
                eprintln!("Unknown error while trying to open file!");
                None
            }
        },
    }
}

fn create_and_start_vm(program_bytes: Vec<u8>, mut available_memory: [u8; MAX_PROGRAM_SIZE], debug_mode: bool) {
    for (i, byte) in program_bytes.iter().enumerate() {
        available_memory[i] = byte.to_owned()
    }

    let mut vm = CrispAte::new(debug_mode);

    println!("Initializing VM...");
    vm.init(available_memory);
    println!("VM initialized!");

    let (mut rl, thread) = create_display();
    let mut history: Vec<String> = Vec::new();

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        draw_frame(vm.screen, d);
        vm.emulation_cyle();

        let state_report = format!("History: \n {:#?} \n Continue execution?", vm.registers.history);

        for report in vm.registers.history {
            history.push(report);
        }

        vm.registers.history = Vec::new();

        if vm.registers.debug_mode == true {
            let choice = dialog::Question::new(state_report)
                .title("CrispAte")
                .show()
                .expect("Could not display dialog box");

            if choice != dialog::Choice::Yes {
                println!("History of rom execution:");
                println!("{:#?}", history);
                panic!("Stopped!")
            }
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

    let available_memory: [u8; 3584] = [0; 3584];

    let program_bytes = get_program_bytes(filename);

    match program_bytes {
        Some(bytes) => {
            if bytes.len() > available_memory.len() {
                eprintln!("File is too big for emulator!");
                std::process::exit(1);
            }

            let choice = dialog::Question::new("Run program in debug mode?")
                .title("CrispAte")
                .show()
                .expect("Could not display dialog box");

            let debug_mode = match choice {
                dialog::Choice::No => false,
                dialog::Choice::Yes => true,
                dialog::Choice::Cancel => false,
            };

            create_and_start_vm(bytes, available_memory, debug_mode)
        }
        None => {
            eprintln!("Failed to get program bytes!");
            std::process::exit(1);
        }
    }
}
