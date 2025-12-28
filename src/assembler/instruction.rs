#[derive(Debug)]
pub enum Instruction {
    Itype {rd: u32, rs1: u32, imm: i32,  opcode: u32, funct3: u32},
}