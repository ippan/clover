use std::collections::HashMap;

use crate::runtime::opcode::{Instruction, OpCode};
use crate::intermediate::{Positions, Position};
use crate::runtime::opcode::OpCode::Jump;

pub type Scope = HashMap<String, usize>;

pub type BreakScope = Vec<usize>;

#[derive(Debug, Clone)]
pub struct FunctionState {
    pub is_instance: bool,
    pub parameter_count: usize,
    pub local_count: usize,
    pub scopes: Vec<Scope>,
    pub break_scopes: Vec<BreakScope>,
    pub instructions: Vec<Instruction>,
    pub positions: Positions
}

impl FunctionState {
    pub fn new() -> FunctionState {
        let mut function_state = FunctionState {
            is_instance: false,
            parameter_count: 0,
            local_count: 0,
            scopes: Vec::new(),
            break_scopes: Vec::new(),
            instructions: Vec::new(),
            positions: Positions::new()
        };

        function_state.enter_scope();

        function_state
    }

    pub fn get_last_position(&self) -> Position {
        if let Some(position) = self.positions.last() {
            position.clone()
        } else {
            Position::none()
        }
    }

    pub fn emit(&mut self, instruction: Instruction, position: Position) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);
        self.positions.push(position);
        index
    }

    pub fn emit_opcode(&mut self, opcode: OpCode, position: Position) -> usize {
        self.emit(opcode.to_instruction(0), position)
    }

    pub fn emit_opcode_without_position(&mut self, opcode: OpCode) -> usize {
        self.emit_opcode(opcode, self.get_last_position())
    }

    pub fn get_next_instruction_index(&self) -> usize {
        self.instructions.len()
    }

    pub fn remove_pop_or_push_null(&mut self) {
        if self.instructions.len() == 0 {
            self.emit_opcode_without_position(OpCode::PushNull);
            return;
        };

        match self.instructions.last().unwrap().opcode() {
            // last statement is a expression statement
            OpCode::Pop => {
                self.instructions.pop();
                self.positions.pop();
            },
            OpCode::Return => {
                // do nothing
            },
            _ => {
                self.emit_opcode_without_position(OpCode::PushNull);
            }
        }
    }

    pub fn emit_return(&mut self, position: Position) {
        self.remove_pop_or_push_null();

        if OpCode::Return != self.instructions.last().unwrap().opcode() {
            self.emit_opcode(OpCode::Return, position);
        };
    }

    pub fn emit_break(&mut self, position: Position) {
        if self.break_scopes.len() == 0 {
            return;
        };

        let index = self.emit_opcode(OpCode::Jump, position);

        if let Some(break_scope) = self.break_scopes.last_mut() {
            break_scope.push(index);
        };
    }

    pub fn replace_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }

    pub fn find_local(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(&index) = scope.get(name) {
                return Some(index);
            };
        }

        None
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn enter_break_scope(&mut self) { self.break_scopes.push(BreakScope::new()); }

    pub fn exit_break_scrope(&mut self) {
        let break_scope = self.break_scopes.pop().unwrap();
        let jump_target = self.get_next_instruction_index() as u64;

        for index in break_scope {
            self.replace_instruction(index, OpCode::Jump.to_instruction(jump_target));
        }

    }

    pub fn define_local(&mut self, name: &str) -> Option<usize> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                return None;
            };

            let index = self.local_count;
            scope.insert(name.to_string(), index);
            self.local_count += 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn define_anonymous_local(&mut self) -> usize {
        let index = self.local_count;
        self.local_count += 1;
        index
    }


}
