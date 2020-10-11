use std::fmt;

#[derive(Copy, Clone)]
pub struct Instruction(u64);

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Pop             = 0x01,

    // operand -> index of constant
    PushConstant    = 0x02,
    PushNull        = 0x03,

    // operand = 0 -> false
    // operand = 1 -> true
    PushBoolean     = 0x04,
    Return          = 0x05,

    // first byte of operand = 0 -> no need to pop (leave value at stack)
    // first byte of operand > 0 -> pop after set
    // operand without first byte -> index of local
    LocalSet        = 0x06,
    // operand -> index of local
    LocalGet        = 0x07,
    EnvironmentSet  = 0x08,
    EnvironmentGet  = 0x09,

    // operand = 0 -> return value
    // operand = 1 -> return instance
    InstanceSet     = 0x0A,
    InstanceGet     = 0x0B,

    Add             = 0x21,
    Sub             = 0x22,
    Multiply        = 0x23,
    Divide          = 0x24,

    Closure         = 0x31,

    // operand -> parameter count
    Call            = 0x32,

    PushNewMap      = 0x41,

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
            0x06 => OpCode::LocalSet,
            0x07 => OpCode::LocalGet,
            0x08 => OpCode::EnvironmentSet,
            0x09 => OpCode::EnvironmentGet,
            0x0A => OpCode::InstanceSet,
            0x0B => OpCode::InstanceGet,

            0x21 => OpCode::Add,
            0x22 => OpCode::Sub,
            0x23 => OpCode::Multiply,
            0x24 => OpCode::Divide,

            0x31 => OpCode::Closure,
            0x32 => OpCode::Call,

            0x41 => OpCode::PushNewMap,

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