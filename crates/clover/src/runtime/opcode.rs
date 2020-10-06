use std::fmt;

#[derive(Copy, Clone)]
pub struct Instruction(u64);

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Pop             = 0x01,
    PushConstant    = 0x02,
    PushNull        = 0x03,
    PushBoolean     = 0x04,
    Return          = 0x05,
    SetLocal        = 0x06,
    GetLocal        = 0x07,

    Unknown         = 0xFF,
}

impl Instruction {
    pub fn form(instruction: u64) -> Instruction {
        Instruction(instruction)
    }

    pub fn opcode(&self) -> OpCode {
        match (self.0 >> 56) & 0xFF {

            0x01 => OpCode::Pop,
            0x02 => OpCode::PushConstant,
            0x03 => OpCode::PushNull,
            0x04 => OpCode::PushBoolean,
            0x05 => OpCode::Return,
            0x06 => OpCode::SetLocal,
            0x07 => OpCode::GetLocal,

            _ => OpCode::Unknown
        }
    }

    pub fn operand(&self) -> u64 {
        self.0 & 0x00FFFFFFFFFFFFFF
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.opcode())
            .field(&self.operand())
            .finish()
    }
}


impl OpCode {
    pub fn to_instruction(&self, operand: u64) -> Instruction {
        Instruction((*self as u64) << 56 | operand)
    }
}