
use risc_v_assembler::assembler::{self, parser};
use risc_v_assembler::utils::{file, exception::AsmRiscVError};

use std::env;

const USAGE: &str = "Usage: cargo run <asm_file> [asm_file] ...";
fn main() {
    let mut instructions = Vec::new();

    let args = env::args().skip(1);

    if args.len() == 0 {
        eprintln!("Error: No input file\n{}", USAGE);
        std::process::exit(1);
    }

    for arg in args {
        match file::read_asm(&arg) {
            Ok(content) => {
                for line in content.lines() {
                    match parser::parse_instruction(line) {
                        Ok(ins) => {
                            println!("{:?}", ins);  
                            instructions.push(ins);
                        }
                        Err(e) => {
                            match e {
                                AsmRiscVError::ParseEmptyLine => continue,
                                _ => {
                                    eprintln!("{:?}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                }

                let binary_contents = assembler::assembly(&instructions);
                println!("{:x?}", binary_contents);
                if let Err(e) = file::write_binary(&arg, &binary_contents) {
                    eprintln!("{:?}", e);
                }
            },

            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    } 
}
