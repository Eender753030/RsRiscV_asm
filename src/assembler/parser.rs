use super::instruction::Instruction;
use crate::utils::exception::AsmRiscVError;

pub fn parse_instruction(line: &str) -> Result<Instruction, AsmRiscVError> {
    let clean_line = line.trim();
    
    if clean_line.is_empty() {
        return Err(AsmRiscVError::ParseEmptyLine);
    }

    let valid_line;

    if let Some(comment) = clean_line.find('#') {
        valid_line = clean_line[..comment].to_lowercase();
        if valid_line.is_empty() {
            return Err(AsmRiscVError::ParseEmptyLine);
        }
    } else {
        valid_line = line.to_lowercase();
    }

    let op_str;
    let last_str;
    match valid_line.split_once(' ') {
        Some((left, right)) => {
            op_str = left;
            last_str = right;
        },
        None => return Err(AsmRiscVError::SyntaxError)
    };

    let mut tokens = last_str.split(',');
    
    match op_str {
        "addi" => {
            Ok(Instruction::Itype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                imm: parse_immediate(tokens.next())?,
                opcode: 0b0010011, 
                funct3: 000
            })
        },

        "add" | "sub" => {
            Ok(Instruction::Rtype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                rs2: parse_register(tokens.next())?,
                opcode: 0b0110011, 
                funct3: 0b000,
                funct7: match op_str {
                    "add" => 0b0000000,
                    "sub" => 0b0100000,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                }
            })
        },

        "sll" | "srl" | "sra"  => {
            Ok(Instruction::Rtype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                rs2: parse_register(tokens.next())?,
                opcode: match op_str {
                    "sll | srl" => 0b0000000,
                    "sra" => 0b0100000,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                }, 
                funct3: match op_str {
                    "sll" => 0b001,
                    "srl" | "sra" => 0b101,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                },
                funct7: 0b0000000
            })
        }

        "slt" | "sltu" | 
        "xor" | "or" | "and" => {
            Ok(Instruction::Rtype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                rs2: parse_register(tokens.next())?,
                opcode: 0b0110011, 
                funct3: match op_str {
                    "slt" => 0b010,
                    "sltu" => 0b011,
                    "xor" => 0b100,
                    "or" => 0b110,
                    "and" => 0b111, 
                    _ => return Err(AsmRiscVError::ParseFunctError)
                },
                funct7: 0b0000000
            })
        }

                }
            })
        }

        _ => {
            Err(AsmRiscVError::NotImplementedInstruction)
        }
    }
}

fn parse_register(reg_token: Option<&str>) -> Result<u32, AsmRiscVError> {
    let reg_str = match reg_token {
        Some(str) => str,
        None => return Err(AsmRiscVError::SyntaxError)
    };

    let clean_reg_str = reg_str.trim();

    if !clean_reg_str.starts_with('x') {
        return Err(AsmRiscVError::SyntaxError);
    }
   
    match clean_reg_str[1..].parse::<u32>() {
        Ok(reg) => {
            if reg > 31 {
                Err(AsmRiscVError::NotExistRegister)
            } else {
                Ok(reg)
            }
        },
        Err(_) => Err(AsmRiscVError::SyntaxError)
    }
}

fn parse_immediate(imm_token: Option<&str>) -> Result<i32, AsmRiscVError> {
    let imm_str = match imm_token {
        Some(str) => str,
        None => return Err(AsmRiscVError::SyntaxError)
    };

    match imm_str.trim().parse::<i32>() {
        Ok(imm) => {
            if imm > 2047 || imm < -2048 {
                Err(AsmRiscVError::ImmediateOverflow)
            } else {
                Ok(imm)
            }
        }
        Err(_) => Err(AsmRiscVError::SyntaxError)
    }
}