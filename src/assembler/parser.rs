use super::instruction::Instruction;
use crate::utils::exception::AsmRiscVError;

pub fn parse_instruction(line: &str) -> Result<Instruction, AsmRiscVError> {
    if line.trim().is_empty() {
        return Err(AsmRiscVError::ParseEmptyLine);
    }

    let valid_line;

    if let Some(comment) = line.find('#') {
        valid_line = line[..comment].to_lowercase();
        if valid_line.trim().is_empty() {
            return Err(AsmRiscVError::ParseEmptyLine);
        }
    } else {
        valid_line = line.to_lowercase();
    }

    let mut tokens = valid_line.split_whitespace();
    
    let opcode = match tokens.next() {
        Some(code) => code,
        None => return Err(AsmRiscVError::SyntaxError)
    };

    match opcode {
        "addi" => {
            Ok(Instruction::Itype {
                rd: parse_register(tokens.next())?,
                rs1: parse_register(tokens.next())?,
                imm: parse_immediate(tokens.next())?,
                opcode: 0b0010011, 
                funct3: 000
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

    if !reg_str.starts_with('x') || !reg_str.ends_with(',') {
        return Err(AsmRiscVError::SyntaxError);
    }
   
    match reg_str[1..].trim_matches(',').parse::<u32>() {
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

    match imm_str.parse::<i32>() {
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