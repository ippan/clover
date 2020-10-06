#[derive(Copy, Clone)]
pub enum OpCode {
    Unknown  = 0x00,
    Constant = 0x01,
    Pop      = 0x02,


}

impl OpCode {
    pub fn form_instruction(instruction: u64) -> OpCode {
        match (instruction >> 56) & 0xFF {

            0x01 => OpCode::Constant,
            0x02 => OpCode::Pop,

            _ => OpCode::Unknown
        }
    }

    pub fn to_instruction(&self, operand: u64) -> u64 {
        (*self as u64) << 56 | operand
    }
}