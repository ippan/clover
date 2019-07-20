using System;
using System.Collections.Generic;

namespace Clover.Runtime
{
    public class Frame
    {
        public ScriptFunction Function;
        public Int32 InstructionPointer = 0;

        public Int32[] local_variable_indices;
        
        public Object LastPop;

        public Object Self;
        
        public Frame(ScriptFunction function, Int32 local_variable_count)
        {
            Function = function;
            local_variable_indices = new Int32[local_variable_count];
        }

        public Int32 CurrentInstruction => Function.Bytecode.Instructions[InstructionPointer];

        public Int32 NextInstruction => Function.Bytecode.Instructions[InstructionPointer + 1];
        
        public void MoveInstructionPointer(int value)
        {
            InstructionPointer += value;
        }

        public void SetInstructionPointer(Int32 value)
        {
            InstructionPointer = value;
        }

        public void SetVariableIndex(int index, Int32 target_index)
        {
            local_variable_indices[index] = target_index;
        }

        public Int32 GetVariableIndex(int index)
        {
            return local_variable_indices[index];
        }

        public Int32 LocalVariableCount => local_variable_indices.Length;
        
        public TokenData LastToken => Function.Bytecode.TokenDatas[InstructionPointer - 1];

        public bool Finished => InstructionPointer >= Function.Bytecode.Instructions.Count;
    }
}