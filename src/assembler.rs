pub mod instruction;
pub mod parser;

use self::instruction::Instruction;

pub fn assembly(instructions: &Vec<Instruction>) -> Vec<u8>{
    let mut binary_contents = Vec::new();
    
    
    for ins in instructions {
        match ins {
            Instruction::Itype {rd, rs1, imm, opcode, funct3} => {
                let ins_u32 = ((imm << 20) as u32) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            },

            Instruction::Rtype {rd, rs1, rs2, opcode, funct3, funct7} => {
                let ins_u32 = (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            }
        }
    }

    binary_contents
}