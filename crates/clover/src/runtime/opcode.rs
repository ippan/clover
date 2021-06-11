use std::fmt;

#[derive(Copy, Clone)]
pub struct Instruction(u64);


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    Pop             = 0x01,

    // operand -> index of constant
    PushConstant    = 0x02,
    PushNull        = 0x03,

    // operand = 0 -> false
    // operand = 1 -> true
    PushBoolean     = 0x04,
    Return          = 0x05,

    // operand -> index of local
    LocalSet        = 0x06,
    // operand -> index of local
    LocalGet        = 0x07,
    // operand -> index of local
    LocalInit       = 0x08,

    ContextSet      = 0x0C,
    ContextGet      = 0x0D,

    // operand -> index of constant (global name)
    GlobalSet       = 0x11,
    // operand -> index of constant (global name)
    GlobalGet       = 0x12,

    InstanceSet     = 0x13,
    InstanceGet     = 0x14,
    IndexSet        = 0x15,
    IndexGet        = 0x16,

    // operand is operator
    Operation       = 0x21,
    Not             = 0x22,
    Negative        = 0x23,

    Closure         = 0x31,

    // operand -> parameter count
    Call            = 0x32,

    // operand -> value count
    Array           = 0x36,

    PushNewMap      = 0x41,

    // operand -> position
    Jump            = 0x51,
    // operand -> position
    JumpIf          = 0x52,

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
            0x08 => OpCode::LocalInit,

            0x0C => OpCode::ContextSet,
            0x0D => OpCode::ContextGet,

            0x11 => OpCode::GlobalSet,
            0x12 => OpCode::GlobalGet,
            0x13 => OpCode::InstanceSet,
            0x14 => OpCode::InstanceGet,
            0x15 => OpCode::IndexSet,
            0x16 => OpCode::IndexGet,


            0x21 => OpCode::Operation,
            0x22 => OpCode::Not,
            0x23 => OpCode::Negative,

            0x31 => OpCode::Closure,
            0x32 => OpCode::Call,

            0x36 => OpCode::Array,

            0x41 => OpCode::PushNewMap,

            0x51 => OpCode::Jump,
            0x52 => OpCode::JumpIf,

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