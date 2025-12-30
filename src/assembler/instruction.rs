#[derive(Debug)]
pub enum Instruction {
    Itype {rd: u32, rs1: u32, imm: i32, opcode: u32, funct3: u32},
    Rtype {rd: u32, rs1: u32, rs2: u32, opcode: u32, funct3: u32, funct7: u32},
    Stype {rs1: u32, rs2: u32, imm: i32, opcode: u32, funct3: u32},
    Btype {rs1: u32, rs2: u32, imm: i32, opcode: u32, funct3: u32},
    Utype {rd: u32, imm: i32, opcode: u32},
    Jtype {rd: u32, imm: i32, opcode: u32},
}