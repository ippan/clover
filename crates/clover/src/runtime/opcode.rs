use std::fmt;

#[derive(Copy, Clone)]
pub struct Instruction(u64);

pub const OPERATION_ADD: usize = 0;
pub const OPERATION_SUB: usize = 1;
pub const OPERATION_MULTIPLY: usize = 2;
pub const OPERATION_DIVIDE: usize = 3;
pub const OPERATION_MOD: usize = 4;
pub const OPERATION_EQUAL: usize = 5;
pub const OPERATION_GREATER: usize = 6;
pub const OPERATION_LESS: usize = 7;
pub const OPERATION_GREATER_EQUAL: usize = 8;
pub const OPERATION_LESS_EQUAL: usize = 9;
pub const OPERATION_AND: usize = 256 | 1;
pub const OPERATION_OR: usize = 256 | 2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    Pop             = 0x01,

    // operand -> index of constant
    PushConstant    = 0x02,

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
    // operand -> enumerable index
    ForNext         = 0x56,
    // operand -> iterator index
    Iterate         = 0x57,

    Unknown         = 0xFF,
}

impl From<u64> for Instruction {
    fn from(instruction: u64) -> Instruction {
        Instruction(instruction)
    }
}

impl Into<u64> for &Instruction {
    fn into(self) -> u64 { self.0 }
}

impl Instruction {

    pub fn opcode(&self) -> OpCode {
        match (self.0 >> 56) & 0xFF {

            0x01 => OpCode::Pop,
            0x02 => OpCode::PushConstant,

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

            0x56 => OpCode::ForNext,
            0x57 => OpCode::Iterate,

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