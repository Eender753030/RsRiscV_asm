
use risc_v_assembler::assembler::{self, parser};
use risc_v_assembler::utils::{file, exception::AsmRiscVError};

fn main() {
    let mut instructions = Vec::new();

    match file::read_asm("test.asm") {
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
            if let Err(e) = file::write_binary("test", &binary_contents) {
                eprintln!("{:?}", e);
            }
        },

        Err(e) => {
            eprintln!("{:?}", e);
        }
    } 
}
