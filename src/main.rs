mod crisp_ate;
mod utils;
use std::env;
use std::io::ErrorKind;
use crisp_ate::cpu::CrispAte;

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

fn create_and_start_vm(program_bytes: Vec<u8>, mut available_memory: [u8; MAX_PROGRAM_SIZE]) {
    for (i, byte) in program_bytes.iter().enumerate() {
        available_memory[i] = byte.to_owned()
    }

    let mut vm = CrispAte::new();

    println!("Initializing VM...");
    vm.init(available_memory);
    println!("VM initialized!");
    println!("");
    println!("---------- PROGRAM STARTING ----------");
    println!("Starting VM registers:");
    println!("{:#?}", vm.registers);
    println!("Starting VM timers:");
    println!("{:#?}", vm.timers);
    println!("Starting VM runtime:");
    println!("{:#?}", vm.runtime);
    println!("");

    loop {
        vm.emulation_cyle();
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

            create_and_start_vm(bytes, available_memory)
        },
        None => {
            eprintln!("Failed to get program bytes!");
            std::process::exit(1);
        }
    }
}
