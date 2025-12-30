pub mod instruction;
pub mod parser;

use self::instruction::Instruction;

/// Turn each instruction type into little endian bytes
pub fn assembly(instructions: &Vec<Instruction>) -> Vec<u8>{
    let mut binary_contents = Vec::new();
    
    for ins in instructions {
        match ins {
            // I-type: imm[11:0] | rs1[4:0] | funct3[2:0] | rd[4:0] | opcode[6:0]
            Instruction::Itype {rd, rs1, imm, opcode, funct3} => {
                let ins_u32 = ((imm << 20) as u32) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            },

            // R-type: funct[6:0] | rs2[4:0] | rs1[4:0] | funct3[2:0] | rd[4:0] | opcode[6:0]
            Instruction::Rtype {rd, rs1, rs2, opcode, funct3, funct7} => {
                let ins_u32 = (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            }

            // S-type: imm[11:5] | rs2[4:0] | rs1[4:0] | funct3[2:0] | imm[4:0] | opcode[6:0]
            Instruction::Stype {rs1, rs2, imm, opcode, funct3} => {
                let ins_u32 = (((imm & 0xfe0) << 20) as u32) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (((imm & 0x1f) << 7) as u32) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            },

            // B-type: imm[12] | imm[10:5] | rs2[4:0] | rs1[4:0] | funct3[2:0] | imm[4:1] | imm[11] | opcode[6:0]
            Instruction::Btype {rs1, rs2, imm, opcode, funct3} => {
                let ins_u32 = ((((imm & 0x1000) << 19) | ((imm & 0x07e0) << 20)) as u32) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | ((((imm & 0x01e) << 7) | ((imm & 0x800) >> 4)) as u32) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            },

            // J-type: imm[20] | imm[10:1] | imm[11] | imm[19:12] | rd[4:0] | opcode[6:0]
            Instruction::Jtype {rd, imm, opcode} => {
                let ins_u32 = ((((imm & 0x100000) << 11) | ((imm & 0x0007fe) << 20) | ((imm & 0x000800) << 9) | (imm & 0x0ff000)) as u32) | (rd << 7) | opcode;
                binary_contents.extend_from_slice(&ins_u32.to_le_bytes());
            }
        }
    }

    binary_contents
}