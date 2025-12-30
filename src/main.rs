
use risc_v_assembler::assembler::{self, parser};
use risc_v_assembler::utils::{file, exception::AsmRiscVError};

use std::env;
use std::collections::HashMap;

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
                let mut label_table = HashMap::<String, i32>::new();
                let clean_content = content.replace(";", "\n");
                let content_iter = clean_content.lines().filter(|s| !s.trim().starts_with('#') && !s.trim().is_empty());
                for (i, line) in content_iter.clone().enumerate(){
                    let ins_line_num = i - label_table.len();
                    if let Err(e) = parser::parse_label(line, &mut label_table, ins_line_num) {     
                        match e {
                            AsmRiscVError::ParseEmptyLine => continue,
                            _ => {
                                eprintln!("{:?}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                }
                
                for line in content_iter {
                    match parser::parse_instruction(line, &label_table, instructions.len()) {
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
                println!("{:x?}\n{:?}", binary_contents, label_table);
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
