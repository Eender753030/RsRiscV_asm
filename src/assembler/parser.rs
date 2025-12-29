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
        "addi" | "slti" | "sltiu" | 
        "xori" | "ori" | "andi" => {
            Ok(Instruction::Itype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                imm: parse_immediate(tokens.next(), false)?,
                opcode: 0b0010011, 
                funct3: match op_str {
                    "addi" => 0b000,
                    "slti" => 0b010,
                    "sltiu" => 0b011,
                    "xori" => 0b100,
                    "ori" => 0b110,
                    "andi" => 0b111, 
                    _ => return Err(AsmRiscVError::ParseFunctError)
                }
            })
        },

        "slli" | "srli" | "srai" => {
            Ok(Instruction::Itype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                imm: (match op_str {
                    "slli" | "srli" => 0b000000,
                    "srai" => 0b0100000,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                } << 5) | (parse_immediate(tokens.next(), true)?),
                opcode: 0b0010011, 
                funct3: match op_str {
                    "slli" => 0b001,
                    "srli" | "srai" => 0b101,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                }
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

        "lb" | "lh" | "lw" | 
        "lbu" | "lhu" => {
            let rd = parse_register(tokens.next())?;
            let (imm, rs1) = parse_ld_or_sd(tokens.next())?;
            Ok(Instruction::Itype {
                rd,
                rs1,
                imm,
                opcode: 0b0000011, 
                funct3: match op_str {
                    "lb" => 0b000,
                    "lh" => 0b001,
                    "lw" => 0b010,
                    "lbu" => 0b100,
                    "lhu" => 0b101,
                    _ => return Err(AsmRiscVError::ParseFunctError)
                }
            })
        },

        "sb" | "sh" | "sw" => {
            let rs2 = parse_register(tokens.next())?;
            let (imm, rs1) = parse_ld_or_sd(tokens.next())?;
            Ok(Instruction::Stype {
                rs2,
                rs1,
                imm,
                opcode: 0b0100011, 
                funct3: match op_str {
                    "sb" => 0b000,
                    "sh" => 0b001,
                    "sw" => 0b010,
                    _ => return Err(AsmRiscVError::ParseFunctError)
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
        Some(str) => str.trim(),
        None => return Err(AsmRiscVError::SyntaxError)
    };

    if !reg_str.starts_with('x') {
        return Err(AsmRiscVError::SyntaxError);
    }
   
    match reg_str[1..].parse::<u32>() {
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

fn parse_immediate(imm_token: Option<&str>, with_funct: bool) -> Result<i32, AsmRiscVError> {
    let imm_str = match imm_token {
        Some(str) => str.trim(),
        None => return Err(AsmRiscVError::SyntaxError)
    };

    let base = match imm_str.as_bytes() {
        [b'0', b'x', ..] => 16,
        [b'0', b'b', ..] => 2,
        [b'0', b'o', ..] => 8,
        _ => 10,
    };
    
    let imm = if base != 10 {
        match u32::from_str_radix(&imm_str[2..], base) {
            Ok(raw) => (raw as i32) << 20 >> 20,          
            Err(_) => return Err(AsmRiscVError::SyntaxError)
        }
    } else {
        match imm_str.trim().parse::<i32>() {
            Ok(imm) => imm,
            Err(_) => return Err(AsmRiscVError::SyntaxError)
        }
    };

    if (with_funct && (imm > 31 || imm < 0)) || 
        (imm > 2047 || imm < -2048) {
        Err(AsmRiscVError::ImmediateOverflow)
    } else {
        Ok(imm)
    }
}

fn parse_ld_or_sd(token: Option<&str>) -> Result<(i32, u32), AsmRiscVError> {
    let token_str = match token {
        Some(str) => str,
        None => return Err(AsmRiscVError::SyntaxError)
    };

    let imm_str;
    let reg_str;

    match token_str.split_once('(') {
        Some((left, right)) => {
            imm_str = left.trim();
            let clean_right = right.trim();
            if !clean_right.starts_with('x') || !clean_right.ends_with(')') {
                return Err(AsmRiscVError::SyntaxError);
            }

            reg_str = clean_right.trim_end_matches(')').trim();
        },

        None => return Err(AsmRiscVError::SyntaxError)
    }

    
    Ok((match imm_str.trim().parse::<i32>() {
        Ok(imm) => {
            if imm > 2047 || imm < -2048 {
                return Err(AsmRiscVError::ImmediateOverflow);
            } else {
                imm
            }
        },
        Err(_) => return Err(AsmRiscVError::SyntaxError)
    }, match reg_str[1..].parse::<u32>() {
        Ok(reg) => {
            if reg > 31 {
                return Err(AsmRiscVError::NotExistRegister);
            } else {
                reg
            }
        },
        Err(_) => return Err(AsmRiscVError::SyntaxError)
    }))
}